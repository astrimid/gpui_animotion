mod clip;
mod clip_ext;
mod element;
mod interpolate;
mod property;
mod track;
mod spring;
mod gravity;
mod tween;

pub use clip::{ClipBuilder, Prop};
pub use clip_ext::AnimotionClipExt;
pub use element::{AnimotionElement, AnimotionExt};
pub use interpolate::Interpolate;
pub use property::{all, prop, IntoTrackGroup, IntoTrackItem, PropertyTrack};
pub use track::{Keyframe, Track};
pub use spring::{SpringParams, spring};
pub use gravity::{GravityParams, gravity};
pub use tween::tween;
