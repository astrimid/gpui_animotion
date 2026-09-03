use crate::segment::AnimationSegment;
use crate::segments::easing::Ease;
use std::fmt::Debug;
use std::time::Duration;

/// Defines how an animation segment's timeline is constrained.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ConstraintMode {
    /// Accelerates or decelerates internal time so the segment's full trajectory
    /// fits exactly within the target duration.
    Fit,
    /// Truncates evaluation at the target duration, freezing execution and
    /// immediately returning the resting equilibrium value.
    Clamp,
}

/// A wrapper segment that adapts any underlying `AnimationSegment<T>` into a fixed duration budget.
pub struct ConstrainedSegment<T> {
    pub inner: Box<dyn AnimationSegment<T>>,
    pub target_duration: Duration,
    pub mode: ConstraintMode,
}

impl<T> Debug for ConstrainedSegment<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConstrainedSegment")
            .field("inner", &self.inner)
            .field("target_duration", &self.target_duration)
            .field("mode", &self.mode)
            .finish()
    }
}

impl<T> ConstrainedSegment<T> {
    pub fn new(
        inner: Box<dyn AnimationSegment<T>>,
        target_duration: Duration,
        mode: ConstraintMode,
    ) -> Self {
        Self {
            inner,
            target_duration,
            mode,
        }
    }

    pub fn fit(inner: Box<dyn AnimationSegment<T>>, target_duration: Duration) -> Self {
        Self::new(inner, target_duration, ConstraintMode::Fit)
    }

    pub fn clamp(inner: Box<dyn AnimationSegment<T>>, target_duration: Duration) -> Self {
        Self::new(inner, target_duration, ConstraintMode::Clamp)
    }
}

impl<T: Clone + Send + Sync + Debug> AnimationSegment<T> for ConstrainedSegment<T> {
    fn duration(&self) -> Duration {
        self.target_duration
    }

    fn evaluate(&self, t: Duration) -> T {
        match self.mode {
            ConstraintMode::Clamp => {
                if t >= self.target_duration {
                    self.inner.end_value()
                } else {
                    self.inner.evaluate(t)
                }
            }
            ConstraintMode::Fit => {
                let natural_secs = self.inner.duration().as_secs_f32();
                let target_secs = self.target_duration.as_secs_f32();

                if target_secs <= 0.0 || natural_secs <= 0.0 {
                    return self.inner.end_value();
                }

                let scale_factor = natural_secs / target_secs;
                let scaled_t = Duration::from_secs_f32(t.as_secs_f32() * scale_factor);
                self.inner.evaluate(scaled_t)
            }
        }
    }

    fn velocity(&self, t: Duration) -> T {
        match self.mode {
            ConstraintMode::Clamp => {
                if t >= self.target_duration {
                    self.inner.velocity(self.inner.duration())
                } else {
                    self.inner.velocity(t)
                }
            }
            ConstraintMode::Fit => {
                let natural_secs = self.inner.duration().as_secs_f32();
                let target_secs = self.target_duration.as_secs_f32();

                if target_secs <= 0.0 || natural_secs <= 0.0 {
                    return self.inner.velocity(Duration::ZERO);
                }

                let scale_factor = natural_secs / target_secs;
                let scaled_t = Duration::from_secs_f32(t.as_secs_f32() * scale_factor);
                self.inner.velocity(scaled_t)
            }
        }
    }

    fn end_value(&self) -> T {
        self.inner.end_value()
    }

    fn set_ease(&mut self, ease: Ease) {
        self.inner.set_ease(ease);
    }
}

/// An inert identity segment that holds a static value for a specified duration.
/// Used for choreography pauses, holds, and initial delays.
#[derive(Clone, Debug)]
pub struct HoldSegment<T> {
    pub value: T,
    pub duration: Duration,
}

impl<T: Clone + Send + Sync + Debug> HoldSegment<T> {
    pub fn new(value: T, duration: Duration) -> Self {
        Self { value, duration }
    }
}

impl<T: Clone + Send + Sync + Debug> AnimationSegment<T> for HoldSegment<T> {
    fn duration(&self) -> Duration {
        self.duration
    }

    fn evaluate(&self, _t: Duration) -> T {
        self.value.clone()
    }

    fn velocity(&self, _t: Duration) -> T {
        self.value.clone()
    }

    fn end_value(&self) -> T {
        self.value.clone()
    }
}
