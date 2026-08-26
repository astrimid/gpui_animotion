use crate::{Interpolate, Track};
use gpui::Div;
use std::time::Duration;

pub trait PropertyTrack: Send + Sync {
    fn apply(&self, el: Div, elapsed: Duration) -> Div;
}

struct AnyPropertyTrack<T> {
    track: Track<T>,
    apply_fn: Box<dyn Fn(Div, T) -> Div + Send + Sync>,
}

impl<T: Clone + Interpolate + Send + Sync + 'static> PropertyTrack for AnyPropertyTrack<T> {
    fn apply(&self, el: Div, elapsed: Duration) -> Div {
        let val = self.track.sample(elapsed);
        (self.apply_fn)(el, val)
    }
}

/// Creates a property binding linking a [`Track`] to a [`Div`] mutation callback.
pub fn prop<T: Clone + Interpolate + Send + Sync + 'static>(
    track: Track<T>,
    apply: impl Fn(Div, T) -> Div + Send + Sync + 'static,
) -> Box<dyn PropertyTrack> {
    Box::new(AnyPropertyTrack {
        track,
        apply_fn: Box::new(apply),
    })
}

pub trait IntoTrackGroup {
    fn into_tracks(self) -> Vec<Box<dyn PropertyTrack>>;
}

impl IntoTrackGroup for Vec<Box<dyn PropertyTrack>> {
    fn into_tracks(self) -> Vec<Box<dyn PropertyTrack>> {
        self
    }
}

pub trait IntoTrackItem {
    fn into_track(self) -> Box<dyn PropertyTrack>;
}

impl IntoTrackItem for Box<dyn PropertyTrack> {
    fn into_track(self) -> Box<dyn PropertyTrack> {
        self
    }
}

macro_rules! impl_into_track_group {
    ( $( $name:ident ),+ $(,)? ) => {
        impl<$( $name: IntoTrackItem ),+> IntoTrackGroup for ( $( $name ),+ , ) {
            fn into_tracks(self) -> Vec<Box<dyn PropertyTrack>> {
                #[allow(non_snake_case)]
                let ( $( $name ),+ , ) = self;
                vec![ $( $name.into_track() ),+ ]
            }
        }
    };
}

impl_into_track_group!(A);
impl_into_track_group!(A, B);
impl_into_track_group!(A, B, C);
impl_into_track_group!(A, B, C, D);
impl_into_track_group!(A, B, C, D, E);
impl_into_track_group!(A, B, C, D, E, F);

/// Combines multiple tracks into a parallel property track list.
pub fn all(group: impl IntoTrackGroup) -> Vec<Box<dyn PropertyTrack>> {
    group.into_tracks()
}
