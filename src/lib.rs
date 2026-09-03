mod clip;
mod clip_ext;
mod element;
mod easing;
mod flick;
mod interpolate;
mod property;
mod track;
mod spring;
mod gravity;
mod tween;
mod segment;
mod segments;
mod loop_mode;

pub use element::{AnimotionElement, AnimotionExt};
pub use property::{all, prop, Prop, PropertyTrack, Animatable};

pub use gravity::{gravity, GravityParams};
pub use flick::{flick, FlickParams};
pub use spring::{spring, SpringParams};
pub use tween::tween;
pub use easing::tween_eased;
pub use segments::{Ease, CubicBezier};

pub use segment::AnimationSegment;
pub use segments::constrained::{ConstrainedSegment, ConstraintMode, HoldSegment};
pub use interpolate::Interpolate;
pub use track::Track;
pub use loop_mode::LoopMode;
pub use clip::ClipBuilder;
pub use clip_ext::AnimotionClipExt;
