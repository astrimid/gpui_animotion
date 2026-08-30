use crate::{AnimationSegment, Interpolate};
use std::time::Duration;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct TweenSegment<T> {
    pub start: T,
    pub target: T,
    pub duration: Duration,
}

impl<T: Clone + Interpolate + Send + Sync + Debug> AnimationSegment<T> for TweenSegment<T> {
    fn duration(&self) -> Duration {
        self.duration
    }

    fn evaluate(&self, t: Duration) -> T {
        if self.duration.is_zero() {
            return self.target.clone();
        }
        let progress = (t.as_secs_f32() / self.duration.as_secs_f32()).clamp(0.0, 1.0);
        self.start.interpolate(&self.target, progress)
    }

    fn velocity(&self, _t: Duration) -> T {
        // Linear velocity is constant: (target - start) / duration
        // For general types, fallback to end interpolation delta
        self.target.clone()
    }

    fn end_value(&self) -> T {
        self.target.clone()
    }
}
