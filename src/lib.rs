mod element;
mod interpolate;
mod property;
mod track;

pub use element::{AnimotionElement, AnimotionExt};
pub use interpolate::Interpolate;
pub use property::{all, prop, IntoTrackGroup, IntoTrackItem, PropertyTrack};
pub use track::{tween, Keyframe, Track};
