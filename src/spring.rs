use crate::{Keyframe, Track};
use std::time::Duration;

/// Helper function to initialize a track driven by spring physics.
pub fn spring(from_y: f32, target: f32, params: SpringParams) -> Track<f32> {
    Track {
        initial: from_y,
        keyframes: Vec::new(),
    }
    .spring(target, params)
}

/// Parameters configuring harmonic spring physics for numeric tracks.
#[derive(Clone, Debug)]
pub struct SpringParams {
    /// Spring stiffness constant (k). Default: 180.0
    pub stiffness: f32,
    /// Friction damping constant (c). Default: 12.0
    pub damping: f32,
    /// Mass of the attached object (m). Default: 1.0
    pub mass: f32,
    /// Initial velocity entering the spring segment. Default: 0.0
    pub initial_velocity: f32,
    /// Settling threshold (epsilon) in units/pixels to consider at rest. Default: 0.05
    pub threshold: f32,
    /// Optional fixed duration. If None, duration is calculated analytically from threshold.
    pub duration: Option<f32>,
    /// Sampling resolution in FPS for keyframe generation. Default: 60.0
    pub sample_fps: f32,
}

impl Default for SpringParams {
    fn default() -> Self {
        Self {
            stiffness: 180.0,
            damping: 12.0,
            mass: 1.0,
            initial_velocity: 0.0,
            threshold: 0.05,
            duration: None,
            sample_fps: 60.0,
        }
    }
}

impl SpringParams {
    /// Helper to configure a spring using stiffness and damping ratio (zeta).
    pub fn from_damping_ratio(stiffness: f32, damping_ratio: f32) -> Self {
        let mass = 1.0;
        let damping = 2.0 * damping_ratio * (stiffness * mass).sqrt();
        Self {
            stiffness,
            damping,
            mass,
            ..Default::default()
        }
    }
}


impl Track<f32> {
    /// Appends an analytical spring trajectory to the track.
    pub fn spring(mut self, target: f32, params: SpringParams) -> Self {
        let start = self.keyframes.last().map(|k| k.target).unwrap_or(self.initial);
        let x0 = start - target;
        let v0 = params.initial_velocity;
        let mass = params.mass.max(0.001);
        let stiffness = params.stiffness.max(0.001);
        let damping = params.damping.max(0.0);
        let threshold = params.threshold.max(0.0001);

        let omega_n = (stiffness / mass).sqrt();
        let zeta = damping / (2.0 * (stiffness * mass).sqrt());

        // 1. Calculate duration analytically if not explicitly forced
        let duration = if let Some(dur) = params.duration {
            dur
        } else if zeta < 0.9999 {
            // Underdamped regime
            let omega_d = omega_n * (1.0 - zeta * zeta).sqrt();
            let a = x0;
            let b = (v0 + zeta * omega_n * x0) / omega_d;
            let amplitude = (a * a + b * b).sqrt();
            if amplitude <= threshold {
                0.0
            } else {
                (amplitude / threshold).ln() / (zeta * omega_n)
            }
        } else if zeta > 1.0001 {
            // Overdamped regime
            let lambda_slow = omega_n * (zeta - (zeta * zeta - 1.0).sqrt());
            if x0.abs() <= threshold {
                0.0
            } else {
                (x0.abs() / threshold).ln() / lambda_slow
            }
        } else {
            // Critically damped regime
            let a = x0;
            let b = v0 + omega_n * x0;
            let r_crit = a.abs() + b.abs() / (omega_n * std::f32::consts::E);
            if r_crit <= threshold {
                0.0
            } else {
                ((r_crit / threshold).ln() + 1.0) / omega_n
            }
        };

        if duration <= 0.0 {
            self.keyframes.push(Keyframe {
                target,
                duration: Duration::from_millis(16),
            });
            return self;
        }

        // 2. Sample the exact analytical formula at dt increments
        let dt = 1.0 / params.sample_fps.max(1.0);
        let step_duration = Duration::from_secs_f32(dt);
        let mut t = dt;

        while t < duration {
            let displacement = if zeta < 0.9999 {
                let omega_d = omega_n * (1.0 - zeta * zeta).sqrt();
                let a = x0;
                let b = (v0 + zeta * omega_n * x0) / omega_d;
                let envelope = (-zeta * omega_n * t).exp();
                envelope * (a * (omega_d * t).cos() + b * (omega_d * t).sin())
            } else if zeta > 1.0001 {
                let omega_d = omega_n * (zeta * zeta - 1.0).sqrt();
                let a = x0;
                let b = (v0 + zeta * omega_n * x0) / omega_d;
                let envelope = (-zeta * omega_n * t).exp();
                envelope * (a * (omega_d * t).cosh() + b * (omega_d * t).sinh())
            } else {
                let a = x0;
                let b = v0 + omega_n * x0;
                let envelope = (-omega_n * t).exp();
                envelope * (a + b * t)
            };

            self.keyframes.push(Keyframe {
                target: target + displacement,
                duration: step_duration,
            });

            t += dt;
        }

        // Final resting frame
        self.keyframes.push(Keyframe {
            target,
            duration: step_duration,
        });

        self
    }
}
