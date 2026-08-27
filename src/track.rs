use crate::Interpolate;
use std::time::Duration;

pub struct Keyframe<T> {
    pub target: T,
    pub duration: Duration,
}

pub struct Track<T> {
    pub initial: T,
    pub keyframes: Vec<Keyframe<T>>,
}

/// Parameters configuring real-world gravitational motion for numeric tracks.
pub struct GravityParams {
    /// Acceleration due to gravity in pixels/s² (e.g. 2500.0).
    pub gravity: f32,
    /// Coefficient of restitution / bounciness elasticity [0.0, 1.0] (e.g. 0.78 for glass).
    pub restitution: f32,
    /// The target floor collision baseline position.
    pub floor_y: f32,
    /// Integration sampling resolution (default 60.0 FPS for smooth linear interpolation).
    pub sample_fps: f32,
}

impl Default for GravityParams {
    fn default() -> Self {
        Self {
            gravity: 2500.0,
            restitution: 0.75,
            floor_y: 300.0,
            sample_fps: 60.0,
        }
    }
}

/// Helper function to initialize a new animation track with a single tween.
pub fn tween<T: Clone>(from: T, to: T, secs: f32) -> Track<T> {
    Track {
        initial: from,
        keyframes: vec![Keyframe {
            target: to,
            duration: Duration::from_secs_f32(secs),
        }],
    }
}

/// Helper function to initialize a track driven by gravitational physics integration.
pub fn gravity(from_y: f32, params: GravityParams, max_bounces: usize) -> Track<f32> {
    Track {
        initial: from_y,
        keyframes: Vec::new(),
    }
    .gravity(params, max_bounces)
}

impl Track<f32> {
    /// Appends a multi-bounce gravity trajectory using physical acceleration integration.
    pub fn gravity(mut self, params: GravityParams, max_bounces: usize) -> Self {
        let dt = 1.0 / params.sample_fps.max(1.0);
        let step_duration = Duration::from_secs_f32(dt);

        let mut current_y = self.keyframes.last().map(|k| k.target).unwrap_or(self.initial);
        let mut vel_y = 0.0f32;
        let mut bounces = 0;

        while bounces < max_bounces {
            // Kinematic integration: v = v + g*dt, y = y + v*dt
            vel_y += params.gravity * dt;
            current_y += vel_y * dt;

            // Floor collision response
            if current_y >= params.floor_y {
                current_y = params.floor_y;
                vel_y = -vel_y * params.restitution;
                bounces += 1;

                // Threshold check: kill negligible micro-vibrations
                if vel_y.abs() < 15.0 {
                    self.keyframes.push(Keyframe {
                        target: params.floor_y,
                        duration: step_duration,
                    });
                    break;
                }
            }

            self.keyframes.push(Keyframe {
                target: current_y,
                duration: step_duration,
            });
        }

        self
    }
}

impl<T: Clone + Interpolate> Track<T> {
    pub fn tween(mut self, target: T, secs: f32) -> Self {
        self.keyframes.push(Keyframe {
            target,
            duration: Duration::from_secs_f32(secs),
        });
        self
    }

    pub fn total_duration(&self) -> Duration {
        self.keyframes.iter().map(|k| k.duration).sum()
    }

    pub fn sample(&self, elapsed: Duration) -> T {
        if self.keyframes.is_empty() {
            return self.initial.clone();
        }

        let total = self.total_duration();
        let looped_elapsed = if total > Duration::ZERO {
            Duration::from_secs_f32(elapsed.as_secs_f32() % total.as_secs_f32())
        } else {
            Duration::ZERO
        };

        let mut accumulated = Duration::ZERO;
        let mut current_start = &self.initial;

        for keyframe in &self.keyframes {
            let next_accumulated = accumulated + keyframe.duration;

            if looped_elapsed < next_accumulated {
                let segment_elapsed = looped_elapsed.saturating_sub(accumulated);
                let progress = (segment_elapsed.as_secs_f32() / keyframe.duration.as_secs_f32()).clamp(0.0, 1.0);
                return current_start.interpolate(&keyframe.target, progress);
            }

            accumulated = next_accumulated;
            current_start = &keyframe.target;
        }

        self.keyframes.last().unwrap().target.clone()
    }
}
