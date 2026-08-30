mod clip;
mod clip_ext;
mod element;
mod interpolate;
mod property;
mod track;
mod spring;
mod gravity;
mod tween;
mod segment;
mod segments;

pub use clip::ClipBuilder;
pub use clip_ext::AnimotionClipExt;
pub use element::{AnimotionElement, AnimotionExt};
pub use interpolate::Interpolate;
pub use property::{all, prop, Prop, PropertyTrack, Animatable};
pub use track::Track;

pub use segment::AnimationSegment;
pub use spring::{spring, SpringParams};
pub use gravity::{gravity, GravityParams};
pub use tween::tween;
