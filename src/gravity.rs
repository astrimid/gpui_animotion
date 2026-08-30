use crate::Track;
use crate::segments::GravitySegment;

/// Parameters configuring gravitational motion and bounce mechanics.
#[derive(Clone, Debug)]
pub struct GravityParams {
    /// Gravitational downward acceleration in px/sec² (e.g., 2400.0)
    pub gravity: f32,
    /// Coefficient of restitution / bounciness elasticity ∈ [0.0, 1.0] (e.g., 0.75)
    pub restitution: f32,
    /// The target floor collision baseline position in pixels
    pub floor_y: f32,
    /// Velocity threshold in px/sec below which bounces terminate. Default: 15.0
    pub rest_threshold: f32,
    /// Initial velocity entering the gravity segment. Default: 0.0
    pub initial_velocity: f32,
}

impl Default for GravityParams {
    fn default() -> Self {
        Self {
            gravity: 2500.0,
            restitution: 0.75,
            floor_y: 300.0,
            rest_threshold: 15.0,
            initial_velocity: 0.0,
        }
    }
}

pub fn gravity(from_y: f32, params: GravityParams, max_bounces: usize) -> Track<f32> {
    Track::new(from_y).gravity(params, max_bounces)
}

impl Track<f32> {
    /// Appends a piecewise parabolic gravity trajectory to the Track
    pub fn gravity(mut self, params: GravityParams, max_bounces: usize) -> Self {
        let start_y = self.current_end_value();
        self.segments.push(Box::new(GravitySegment::new(
            start_y,
            params,
            max_bounces,
        )));
        self
    }
}
