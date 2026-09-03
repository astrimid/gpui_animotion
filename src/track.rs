use crate::{AnimationSegment, Interpolate};
use crate::segments::constrained::{ConstrainedSegment, HoldSegment};
use crate::loop_mode::LoopMode;
use std::time::Duration;
use std::fmt::Debug;

pub struct Track<T> {
    pub initial: T,
    pub segments: Vec<Box<dyn AnimationSegment<T>>>,
    pub loop_mode: LoopMode,
}

impl<T: Clone> Track<T> {
    pub fn current_end_value(&self) -> T {
        self.segments
            .last()
            .map(|s| s.end_value())
            .unwrap_or_else(|| self.initial.clone())
    }

    pub fn total_duration(&self) -> Duration {
        self.segments.iter().map(|s| s.duration()).sum()
    }
}

impl<T: Clone + Interpolate + Send + Sync + Debug + 'static> Track<T> {
    pub fn new(initial: T) -> Self {
        Self {
            initial,
            segments: Vec::new(),
            loop_mode: LoopMode::default(),
        }
    }

    /// Evaluates the track at elapsed time, cycling across loops
    pub fn sample(&self, elapsed: Duration) -> T {
        if self.segments.is_empty() {
            return self.initial.clone();
        }

        let total = self.total_duration();
        let local_elapsed = self.loop_mode.map_time(elapsed, total);

        let mut accumulated = Duration::ZERO;
        for segment in &self.segments {
            let next_accumulated = accumulated + segment.duration();
            if local_elapsed < next_accumulated {
                let local_t = local_elapsed.saturating_sub(accumulated);
                return segment.evaluate(local_t);
            }
            accumulated = next_accumulated;
        }

        self.segments.last().unwrap().end_value()
    }

    /// Samples both position and instantaneous velocity at timestamp t
    pub fn sample_state(&self, elapsed: Duration) -> (T, T) {
        if self.segments.is_empty() {
            return (self.initial.clone(), self.initial.clone());
        }

        let total = self.total_duration();
        let local_elapsed = self.loop_mode.map_time(elapsed, total);

        let mut accumulated = Duration::ZERO;
        for segment in &self.segments {
            let next_accumulated = accumulated + segment.duration();
            if local_elapsed < next_accumulated {
                let local_t = local_elapsed.saturating_sub(accumulated);
                return (segment.evaluate(local_t), segment.velocity(local_t));
            }
            accumulated = next_accumulated;
        }

        let last = self.segments.last().unwrap();
        (last.end_value(), last.velocity(last.duration()))
    }

    /// Fits the most recently appended segment into an exact duration by scaling its timeline.
    pub fn fit_to_duration(mut self, secs: f32) -> Self {
        if let Some(last) = self.segments.pop() {
            self.segments.push(Box::new(ConstrainedSegment::fit(
                last,
                Duration::from_secs_f32(secs),
            )));
        }
        self
    }

    /// Clamps the most recently appended segment to a maximum duration, snapping to rest once exceeded.
    pub fn clamp_at_duration(mut self, secs: f32) -> Self {
        if let Some(last) = self.segments.pop() {
            self.segments.push(Box::new(ConstrainedSegment::clamp(
                last,
                Duration::from_secs_f32(secs),
            )));
        }
        self
    }

    // --- Delays and Choreography Holds ---

    /// Appends an inert hold segment maintaining the previous value for the given duration.
    pub fn hold(mut self, secs: f32) -> Self {
        let start = self.current_end_value();
        self.segments.push(Box::new(HoldSegment::new(
            start,
            Duration::from_secs_f32(secs),
        )));
        self
    }

    /// Alias for `.hold()`, used at the start or middle of track choreography.
    pub fn delay(self, secs: f32) -> Self {
        self.hold(secs)
    }

    // --- Loop Policies ---

    pub fn loop_mode(mut self, mode: LoopMode) -> Self {
        self.loop_mode = mode;
        self
    }

    pub fn loop_forever(self) -> Self {
        self.loop_mode(LoopMode::LoopForever)
    }

    pub fn play_once(self) -> Self {
        self.loop_mode(LoopMode::Once)
    }

    pub fn loop_count(self, count: usize) -> Self {
        self.loop_mode(LoopMode::Count(count))
    }

    pub fn ping_pong(self) -> Self {
        self.loop_mode(LoopMode::PingPong)
    }

    pub fn ping_pong_count(self, count: usize) -> Self {
        self.loop_mode(LoopMode::PingPongCount(count))
    }
}

