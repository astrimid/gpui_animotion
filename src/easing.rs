use crate::{Track, Animatable, Ease};
use crate::segments::TweenSegment;
use std::time::Duration;

/// Helper function to initialize a new Track with an eased tween.
pub fn tween_eased<T: Animatable>(
    from: T,
    to: T,
    secs: f32,
    ease: Ease,
) -> Track<T> {
    Track::new(from).tween_eased(to, secs, ease)
}

impl<T: Animatable> Track<T> {
    /// Modifies the easing profile of the most recently added segment.
    pub fn ease(mut self, ease: Ease) -> Self {
        if let Some(last) = self.segments.last_mut() {
            last.set_ease(ease);
        }
        self
    }

    /// Appends a tween segment with a specific easing profile in a single call.
    pub fn tween_eased(mut self, target: T, secs: f32, ease: Ease) -> Self {
        let start = self.current_end_value();
        self.segments.push(Box::new(
            TweenSegment::new(start, target, Duration::from_secs_f32(secs)).with_ease(ease),
        ));
        self
    }
}
