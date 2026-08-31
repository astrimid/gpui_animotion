use crate::{AnimationSegment, Interpolate};
use crate::segments::Ease;
use std::time::Duration;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct TweenSegment<T> {
    pub start: T,
    pub target: T,
    pub duration: Duration,
    ease: Ease,
}

impl<T: Clone + Interpolate + Send + Sync + Debug> TweenSegment<T> {
    pub fn new(start: T, target: T, duration: Duration) -> Self {
        Self {
            start,
            target,
            duration,
            ease: Ease::Linear,
        }
    }

    pub fn with_ease(mut self, ease: Ease) -> Self {
        self.ease = ease;
        self
    }
}

impl<T: Clone + Interpolate + Send + Sync + Debug> AnimationSegment<T> for TweenSegment<T> {
    fn duration(&self) -> Duration {
        self.duration
    }

    fn evaluate(&self, t: Duration) -> T {
        if self.duration.is_zero() {
            return self.target.clone();
        }
        let raw_progress = (t.as_secs_f32() / self.duration.as_secs_f32()).clamp(0.0, 1.0);
        let eased_progress = self.ease.sample(raw_progress);
        self.start.interpolate(&self.target, eased_progress)
    }

    fn velocity(&self, t: Duration) -> T {
        if self.duration.is_zero() {
            return self.target.clone();
        }
        let raw_progress = (t.as_secs_f32() / self.duration.as_secs_f32()).clamp(0.0, 1.0);
        let ease_derivative = self.ease.derivative(raw_progress);

        // Instantaneous slope scaled by total trajectory delta
        self.start.interpolate(&self.target, (raw_progress * ease_derivative).clamp(0.0, 1.0))
    }

    fn end_value(&self) -> T {
        self.target.clone()
    }

    fn set_ease(&mut self, ease: Ease) {
        self.ease = ease;
    }
}
