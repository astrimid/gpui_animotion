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

/// Helper function to initialize a new animation track.
pub fn tween<T: Clone>(from: T, to: T, secs: f32) -> Track<T> {
    Track {
        initial: from,
        keyframes: vec![Keyframe {
            target: to,
            duration: Duration::from_secs_f32(secs),
        }],
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
