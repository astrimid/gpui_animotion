mod clip;
mod clip_ext;
mod element;
mod interpolate;
mod property;
mod track;

pub use clip::{ClipBuilder, Prop};
pub use clip_ext::AnimotionClipExt;
pub use element::{AnimotionElement, AnimotionExt};
pub use interpolate::Interpolate;
pub use property::{all, prop, IntoTrackGroup, IntoTrackItem, PropertyTrack};
pub use track::{tween, gravity, GravityParams, Keyframe, Track};
