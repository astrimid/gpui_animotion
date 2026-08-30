use crate::segment::AnimationSegment;
use crate::FlickParams;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct FlickSegment {
    pub start: f32,
    pub v0: f32,
    pub friction: f32,
    pub duration: Duration,
    pub end_value: f32,
}

impl FlickSegment {
    pub fn new(start: f32, params: FlickParams) -> Self {
        let v0 = params.initial_velocity;
        let threshold = params.threshold.max(0.001);

        // If initial velocity is below the resting threshold, settle immediately
        if v0.abs() <= threshold {
            return Self {
                start,
                v0,
                friction: params.friction.max(0.001),
                duration: Duration::ZERO,
                end_value: start,
            };
        }

        // Determine friction coefficient (c) and duration
        let (c, dur_secs) = if let Some(dur) = params.duration {
            let clamped_dur = dur.max(0.001);
            // Solve: |v0| * exp(-c * dur) = threshold => c = ln(|v0| / threshold) / dur
            let solved_c = (v0.abs() / threshold).ln() / clamped_dur;
            (solved_c.max(0.001), clamped_dur)
        } else {
            let c = params.friction.max(0.001);
            let natural_dur = (v0.abs() / threshold).ln() / c;
            (c, natural_dur.max(0.0))
        };

        let duration = Duration::from_secs_f32(dur_secs);
        let end_value = start + (v0 / c) * (1.0 - (-c * dur_secs).exp());

        Self {
            start,
            v0,
            friction: c,
            duration,
            end_value,
        }
    }
}

impl AnimationSegment<f32> for FlickSegment {
    fn duration(&self) -> Duration {
        self.duration
    }

    fn evaluate(&self, t: Duration) -> f32 {
        let t_sec = t.as_secs_f32();
        if t_sec <= 0.0 {
            return self.start;
        }
        if t >= self.duration || self.duration.is_zero() {
            return self.end_value;
        }

        self.start + (self.v0 / self.friction) * (1.0 - (-self.friction * t_sec).exp())
    }

    fn velocity(&self, t: Duration) -> f32 {
        let t_sec = t.as_secs_f32();
        if t_sec <= 0.0 {
            return self.v0;
        }
        if t >= self.duration || self.duration.is_zero() {
            return 0.0;
        }

        self.v0 * (-self.friction * t_sec).exp()
    }

    fn end_value(&self) -> f32 {
        self.end_value
    }
}
