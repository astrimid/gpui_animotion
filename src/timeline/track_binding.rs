use crate::interpolate::Interpolate;
use crate::property::Prop;
use crate::LoopMode;
use std::fmt::Debug;
use std::time::Duration;

/// Trait for type-erased timeline playback and scrubbing control.
pub trait TimelineTrack: Send + Sync + Debug {
    /// Total duration of this track, including its start offset.
    fn duration(&self) -> Duration;

    /// Evaluates the property at the master timeline timestamp `t`.
    fn seek(&self, t: Duration);

    /// Optional identifier for cue points and track-relative sequencing.
    fn name(&self) -> Option<&str>;

    /// The start timestamp offset of this track along the master timeline.
    fn start_offset(&self) -> Duration;
}

/// A concrete property track bound to a start offset along the master timeline.
#[derive(Clone, Debug)]
pub struct BoundTrack<T> {
    pub name: Option<String>,
    pub start_offset: Duration,
    pub prop: Prop<T>,
}

impl<T> BoundTrack<T> {
    pub fn new(name: Option<String>, start_offset: Duration, prop: Prop<T>) -> Self {
        // Enforce LoopMode::Once so the property holds its end value once complete
        prop.track.lock().unwrap().loop_mode = LoopMode::Once;
        Self {
            name,
            start_offset,
            prop,
        }
    }
}

impl<T: Clone + Interpolate + Send + Sync + Debug + 'static> TimelineTrack for BoundTrack<T> {
    fn duration(&self) -> Duration {
        let inner_duration = self.prop.track.lock().unwrap().total_duration();
        self.start_offset + inner_duration
    }

    fn seek(&self, t: Duration) {
        if t < self.start_offset {
            // Before this track begins: evaluate at its starting value
            self.prop.update(Duration::ZERO);
        } else {
            let local_t = t - self.start_offset;
            let inner_dur = self.prop.track.lock().unwrap().total_duration();
            // Clamp to inner duration so it holds its end value once finished
            let clamped_local_t = local_t.min(inner_dur);
            self.prop.update(clamped_local_t);
        }
    }

    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    fn start_offset(&self) -> Duration {
        self.start_offset
    }
}
