use crate::{AnimationSegment, Interpolate};
use std::time::Duration;
use std::fmt::Debug;

pub struct Track<T> {
    pub initial: T,
    pub segments: Vec<Box<dyn AnimationSegment<T>>>,
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
        }
    }

    /// Evaluates the track at elapsed time, cycling across loops
    pub fn sample(&self, elapsed: Duration) -> T {
        if self.segments.is_empty() {
            return self.initial.clone();
        }

        let total = self.total_duration();
        let looped_elapsed = if total > Duration::ZERO {
            Duration::from_secs_f32(elapsed.as_secs_f32() % total.as_secs_f32())
        } else {
            Duration::ZERO
        };

        let mut accumulated = Duration::ZERO;
        for segment in &self.segments {
            let next_accumulated = accumulated + segment.duration();
            if looped_elapsed < next_accumulated {
                let local_t = looped_elapsed.saturating_sub(accumulated);
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
        let looped_elapsed = if total > Duration::ZERO {
            Duration::from_secs_f32(elapsed.as_secs_f32() % total.as_secs_f32())
        } else {
            Duration::ZERO
        };

        let mut accumulated = Duration::ZERO;
        for segment in &self.segments {
            let next_accumulated = accumulated + segment.duration();
            if looped_elapsed < next_accumulated {
                let local_t = looped_elapsed.saturating_sub(accumulated);
                return (segment.evaluate(local_t), segment.velocity(local_t));
            }
            accumulated = next_accumulated;
        }

        let last = self.segments.last().unwrap();
        (last.end_value(), last.velocity(last.duration()))
    }
}

