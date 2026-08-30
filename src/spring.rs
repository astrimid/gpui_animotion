use crate::Track;
use crate::segments::SpringSegment;

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

pub fn spring(from: f32, to: f32, params: crate::SpringParams) -> Track<f32> {
    Track::new(from).spring(to, params)
}

impl Track<f32> {
    pub fn spring(mut self, target: f32, params: crate::SpringParams) -> Self {
        let start = self.current_end_value();
        self.segments.push(Box::new(SpringSegment::new(start, target, params)));
        self
    }
}
