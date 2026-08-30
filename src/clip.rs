use crate::interpolate::Interpolate;
use crate::property::{Prop, PropertyTrack};
use crate::track::Track;
use gpui::Div;
use std::fmt::Debug;
use std::time::Duration;

/// Dynamic track that samples all registered properties and applies the render closure.
pub struct RenderClipTrack<P, Render> {
    props: P,
    render_fn: Render,
    update_props_fn: Box<dyn Fn(&P, Duration) + Send + Sync>,
}

impl<P: Send + Sync + 'static, Render: Fn(Div, &P) -> Div + Send + Sync + 'static> PropertyTrack
    for RenderClipTrack<P, Render>
{
    fn apply(&self, el: Div, elapsed: Duration) -> Div {
        // 1. Update all prop samples for the current frame
        (self.update_props_fn)(&self.props, elapsed);
        // 2. Re-apply the render styling closure to the Div
        (self.render_fn)(el, &self.props)
    }
}

/// Builder responsible for collecting clip properties and their update listeners.
pub struct ClipBuilder {
    tracks: Vec<Box<dyn PropertyTrack>>,
    prop_updaters: Vec<Box<dyn Fn(Duration) + Send + Sync>>,
}

impl ClipBuilder {
    pub fn new() -> Self {
        Self {
            tracks: Vec::new(),
            prop_updaters: Vec::new(),
        }
    }

    /// Creates a tracked property initialized with a scalar value.
    pub fn prop<T: Clone + Interpolate + Send + Sync + Debug + 'static>(&mut self, initial: T) -> Prop<T> {
        let prop_handle = Prop::new(initial);
        let prop_clone = prop_handle.clone();

        self.prop_updaters
            .push(Box::new(move |elapsed| prop_clone.update(elapsed)));

        prop_handle
    }

    /// Creates a tracked property from a pre-configured `Track<T>`.
    pub fn track<T: Clone + Interpolate + Send + Sync + Debug + 'static>(&mut self, track: Track<T>) -> Prop<T> {
        let prop_handle = Prop::from_track(track);
        let prop_clone = prop_handle.clone();

        self.prop_updaters
            .push(Box::new(move |elapsed| prop_clone.update(elapsed)));

        prop_handle
    }

    /// Links the collected properties to the render closure and registers the clip track.
    pub fn register_render<P, Render>(&mut self, props: P, render: Render)
    where
        P: Send + Sync + 'static,
        Render: Fn(Div, &P) -> Div + Send + Sync + 'static,
    {
        let updaters = std::mem::take(&mut self.prop_updaters);

        let update_props_fn = Box::new(move |_: &P, elapsed: Duration| {
            for updater in &updaters {
                updater(elapsed);
            }
        });

        self.tracks.push(Box::new(RenderClipTrack {
            props,
            render_fn: render,
            update_props_fn,
        }));
    }

    /// Consumes the builder and returns the compiled list of property tracks.
    pub fn build(self) -> Vec<Box<dyn PropertyTrack>> {
        self.tracks
    }
}
