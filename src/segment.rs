use crate::segments::Ease;
use std::fmt::Debug;
use std::time::Duration;

/// Core evaluation contract for all time-parametric trajectories
pub trait AnimationSegment<T>: Send + Sync + Debug {
    /// Total duration this segment occupies on the timeline
    fn duration(&self) -> Duration;

    /// Evaluates the property value at localized time offset t ∈ [0, duration]
    fn evaluate(&self, t: Duration) -> T;

    /// Evaluates the instantaneous rate of change (velocity) at localized time offset t
    fn velocity(&self, t: Duration) -> T;

    /// Returns the terminal/settled resting value at t = duration
    fn end_value(&self) -> T;

    /// Configures the easing curve if this segment supports non-linear transfer functions.
    fn set_ease(&mut self, _ease: Ease) {}
}
