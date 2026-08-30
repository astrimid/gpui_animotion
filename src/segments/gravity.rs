use crate::segment::AnimationSegment;
use crate::GravityParams;
use std::time::Duration;

#[derive(Clone, Copy, Debug)]
struct BounceArc {
    start_time: f32,
    duration: f32,
    y0: f32,
    v0: f32,
}

#[derive(Clone, Debug)]
pub struct GravitySegment {
    pub start_y: f32,
    pub floor_y: f32,
    pub gravity: f32,
    pub total_duration: Duration,
    arcs: Vec<BounceArc>,
}

impl GravitySegment {
    pub fn new(start_y: f32, params: GravityParams, max_bounces: usize) -> Self {
        let g = params.gravity.max(1.0);
        let floor = params.floor_y;
        let restitution = params.restitution.clamp(0.0, 0.999);
        let rest_threshold = params.rest_threshold.max(0.1);

        let mut arcs = Vec::with_capacity(max_bounces + 1);
        let mut accumulated_time = 0.0f32;

        // -------------------------------------------------------------
        // ARC 0: Initial Drop from start_y to floor_y
        // -------------------------------------------------------------
        let mut cur_v0 = params.initial_velocity;
        let delta_y = floor - start_y;

        if delta_y > 0.001 {
            let discriminant = (cur_v0 * cur_v0 + 2.0 * g * delta_y).max(0.0);
            let t_drop = (-cur_v0 + discriminant.sqrt()) / g;

            if t_drop > 0.0 {
                arcs.push(BounceArc {
                    start_time: accumulated_time,
                    duration: t_drop,
                    y0: start_y,
                    v0: cur_v0,
                });

                accumulated_time += t_drop;
                // Velocity upon hitting the floor:
                let v_impact = cur_v0 + g * t_drop;
                // Rebound velocity directed upward (negative in screen coordinates):
                cur_v0 = -v_impact * restitution;
            }
        }

        // -------------------------------------------------------------
        // ARCS 1..N: Subsequent Parabolic Rebounds from the Floor
        // -------------------------------------------------------------
        let mut bounce_count = 0;
        while bounce_count < max_bounces && cur_v0.abs() >= rest_threshold {
            // Full ballistic flight time until hitting floor again:
            // floor + v0*t + 0.5*g*t^2 = floor  =>  t*(v0 + 0.5*g*t) = 0  =>  t = -2*v0 / g
            let t_flight = (-2.0 * cur_v0) / g;

            if t_flight <= 0.0001 {
                break;
            }

            arcs.push(BounceArc {
                start_time: accumulated_time,
                duration: t_flight,
                y0: floor,
                v0: cur_v0,
            });

            accumulated_time += t_flight;
            let v_impact = cur_v0 + g * t_flight;
            cur_v0 = -v_impact * restitution;
            bounce_count += 1;
        }

        Self {
            start_y,
            floor_y: floor,
            gravity: g,
            total_duration: Duration::from_secs_f32(accumulated_time),
            arcs,
        }
    }
}

impl AnimationSegment<f32> for GravitySegment {
    fn duration(&self) -> Duration {
        self.total_duration
    }

    fn evaluate(&self, t: Duration) -> f32 {
        let t_sec = t.as_secs_f32();
        if t_sec <= 0.0 {
            return self.start_y;
        }
        if t >= self.total_duration || self.arcs.is_empty() {
            return self.floor_y;
        }

        // Find the active arc using a fast linear scan (arcs is typically <= 8 elements)
        for arc in &self.arcs {
            if t_sec < arc.start_time + arc.duration {
                let local_t = t_sec - arc.start_time;
                let computed_y = arc.y0 + arc.v0 * local_t + 0.5 * self.gravity * local_t * local_t;
                return computed_y.min(self.floor_y); // Prevent sub-pixel penetration below floor
            }
        }

        self.floor_y
    }

    fn velocity(&self, t: Duration) -> f32 {
        let t_sec = t.as_secs_f32();
        if t_sec <= 0.0 {
            return self.arcs.first().map(|a| a.v0).unwrap_or(0.0);
        }
        if t >= self.total_duration || self.arcs.is_empty() {
            return 0.0;
        }

        for arc in &self.arcs {
            if t_sec < arc.start_time + arc.duration {
                let local_t = t_sec - arc.start_time;
                return arc.v0 + self.gravity * local_t;
            }
        }

        0.0
    }

    fn end_value(&self) -> f32 {
        self.floor_y
    }
}
