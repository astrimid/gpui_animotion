use crate::{Keyframe, Track};
use std::time::Duration;

/// Helper function to initialize a new animation track with a single tween.
pub fn tween<T: Clone>(from: T, to: T, secs: f32) -> Track<T> {
    Track {
        initial: from,
        keyframes: vec![Keyframe {
            target: to,
            duration: Duration::from_secs_f32(secs),
        }],
    }
}

impl<T> Track<T> {
    pub fn tween(mut self, target: T, secs: f32) -> Self {
        self.keyframes.push(Keyframe {
            target,
            duration: Duration::from_secs_f32(secs),
        });
        self
    }
}
