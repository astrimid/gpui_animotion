use crate::{GravityParams, Interpolate, PropertyTrack, Track};
use gpui::Div;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Clone)]
pub struct Prop<T> {
    track: Arc<Mutex<Track<T>>>,
    current_value: Arc<Mutex<T>>,
}

impl<T: Clone + Interpolate + Send + Sync + 'static> Prop<T> {
    /// Constructs a new property starting from a single initial scalar value.
    pub fn new(initial: T) -> Self {
        Self {
            track: Arc::new(Mutex::new(Track {
                initial: initial.clone(),
                keyframes: Vec::new(),
            })),
            current_value: Arc::new(Mutex::new(initial)),
        }
    }

    /// Constructs a new property directly from a pre-calculated `Track<T>`.
    pub fn from_track(track: Track<T>) -> Self {
        let initial = track.initial.clone();
        Self {
            track: Arc::new(Mutex::new(track)),
            current_value: Arc::new(Mutex::new(initial)),
        }
    }

    pub fn tween(&self, target: T, secs: f32) -> &Self {
        let mut track = self.track.lock().unwrap();
        track.keyframes.push(crate::Keyframe {
            target,
            duration: Duration::from_secs_f32(secs),
        });
        self
    }

    pub fn get(&self) -> T {
        self.current_value.lock().unwrap().clone()
    }

    pub fn update(&self, elapsed: Duration) {
        let track = self.track.lock().unwrap();
        let sampled = track.sample(elapsed);
        *self.current_value.lock().unwrap() = sampled;
    }
}

impl Prop<f32> {
    /// Appends a gravity bounce trajectory to an existing numeric Prop.
    pub fn gravity(&self, params: GravityParams, max_bounces: usize) -> &Self {
        let mut track = self.track.lock().unwrap();
        // Mutates the underlying Track<f32> by extending keyframes with gravity integration
        let updated_track = std::mem::replace(
            &mut *track,
            Track {
                initial: 0.0,
                keyframes: Vec::new(),
            },
        )
        .gravity(params, max_bounces);

        *track = updated_track;
        self
    }
}

/// Dynamic Track that updates prop samples AND applies the render closure every frame
struct RenderClipTrack<P, Render> {
    props: P,
    render_fn: Render,
    update_props_fn: Box<dyn Fn(&P, Duration) + Send + Sync>,
}

impl<P: Send + Sync + 'static, Render: Fn(Div, &P) -> Div + Send + Sync + 'static> PropertyTrack
    for RenderClipTrack<P, Render>
{
    fn apply(&self, el: Div, elapsed: Duration) -> Div {
        // 1. Sample all tracks for the current time delta
        (self.update_props_fn)(&self.props, elapsed);
        // 2. Re-apply the render styling closure to the Div
        (self.render_fn)(el, &self.props)
    }
}

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

    /// Creates a property from a scalar initial value.
    pub fn prop<T: Clone + Interpolate + Send + Sync + 'static>(&mut self, initial: T) -> Prop<T> {
        let prop_handle = Prop::new(initial);
        let prop_clone = prop_handle.clone();

        self.prop_updaters
            .push(Box::new(move |elapsed| prop_clone.update(elapsed)));

        prop_handle
    }

    /// Creates a property directly from an animation `Track<T>`.
    pub fn track<T: Clone + Interpolate + Send + Sync + 'static>(&mut self, track: Track<T>) -> Prop<T> {
        let prop_handle = Prop::from_track(track);
        let prop_clone = prop_handle.clone();

        self.prop_updaters
            .push(Box::new(move |elapsed| prop_clone.update(elapsed)));

        prop_handle
    }

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

    pub fn build(self) -> Vec<Box<dyn PropertyTrack>> {
        self.tracks
    }
}
