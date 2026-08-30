mod clip;
mod clip_ext;
mod element;
mod flick;
mod interpolate;
mod property;
mod track;
mod spring;
mod gravity;
mod tween;
mod segment;
mod segments;

pub use element::{AnimotionElement, AnimotionExt};
pub use property::{all, prop, Prop, PropertyTrack, Animatable};

pub use gravity::{gravity, GravityParams};
pub use flick::{flick, FlickParams};
pub use spring::{spring, SpringParams};
pub use tween::tween;

pub use segment::AnimationSegment;
pub use interpolate::Interpolate;
pub use track::Track;
pub use clip::ClipBuilder;
pub use clip_ext::AnimotionClipExt;
