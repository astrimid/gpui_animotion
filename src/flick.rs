use crate::Track;
use crate::segments::FlickSegment;

/// Configuration parameters for kinetic friction decay.
#[derive(Clone, Debug)]
pub struct FlickParams {
    /// Initial velocity in px/sec or units/sec entering the segment (v0).
    pub initial_velocity: f32,
    /// Friction / drag coefficient (c). Higher values stop faster. Default: 4.2
    pub friction: f32,
    /// Rest threshold velocity in px/sec below which movement stops. Default: 0.5
    pub threshold: f32,
    /// Optional forced duration limit. If set, friction `c` is solved automatically.
    pub duration: Option<f32>,
}

impl Default for FlickParams {
    fn default() -> Self {
        Self {
            initial_velocity: 0.0,
            friction: 4.2,
            threshold: 0.5,
            duration: None,
        }
    }
}

impl FlickParams {
    /// Helper to create a timed flick where friction is derived to match a duration.
    pub fn timed(initial_velocity: f32, duration_secs: f32) -> Self {
        Self {
            initial_velocity,
            duration: Some(duration_secs),
            ..Default::default()
        }
    }
}

/// Helper function to initialize a new Track driven by kinetic flick decay.
pub fn flick(from: f32, params: FlickParams) -> Track<f32> {
    Track::new(from).flick(params)
}

impl Track<f32> {
    /// Appends an analytical kinetic friction decay segment to the Track.
    pub fn flick(mut self, params: FlickParams) -> Self {
        let start = self.current_end_value();
        self.segments.push(Box::new(FlickSegment::new(start, params)));
        self
    }
}

