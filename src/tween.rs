use crate::{Track, Animatable};
use crate::segments::TweenSegment;
use std::time::Duration;

pub fn tween<T: Animatable>(
    from: T,
    to: T,
    secs: f32,
) -> Track<T> {
    Track::new(from).tween(to, secs)
}

impl<T: Animatable> Track<T> {
    pub fn tween(mut self, target: T, secs: f32) -> Self {
        let start = self.current_end_value();
        self.segments.push(Box::new(TweenSegment {
            start,
            target,
            duration: Duration::from_secs_f32(secs),
        }));
        self
    }
}
