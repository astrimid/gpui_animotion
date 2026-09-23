use crate::interpolate::Interpolate;
use crate::{GravityParams, FlickParams, SpringParams, Ease};
use crate::segments::{TweenSegment, SpringSegment, GravitySegment, FlickSegment};
use crate::segments::{ConstrainedSegment, HoldSegment};
use crate::track::Track;
use gpui::Div;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use crate::LoopMode;
use std::fmt::{self, Debug};

/// Trait for applying animated properties to GPUI elements.
pub trait PropertyTrack: Send + Sync {
    fn apply(&self, el: Div, elapsed: Duration) -> Div;
}

/// Dynamic track linking a single Track<T> to an inline Div modifier callback.
pub struct AnyPropertyTrack<T> {
    pub track: Track<T>,
    pub apply_fn: Box<dyn Fn(Div, T) -> Div + Send + Sync>,
}

impl<T: Clone + Interpolate + Send + Sync + Debug + 'static> PropertyTrack for AnyPropertyTrack<T> {
    fn apply(&self, el: Div, elapsed: Duration) -> Div {
        let val = self.track.sample(elapsed);
        (self.apply_fn)(el, val)
    }
}

/// Helper function to create a PropertyTrack from a Track<T> and an inline modifier closure.
pub fn prop<T: Clone + Interpolate + Send + Sync + Debug + 'static>(
    track: Track<T>,
    apply: impl Fn(Div, T) -> Div + Send + Sync + 'static,
) -> Box<dyn PropertyTrack> {
    Box::new(AnyPropertyTrack {
        track,
        apply_fn: Box::new(apply),
    })
}

/// Reactive property handle used in clips and canvas animations.
#[derive(Clone)]
pub struct Prop<T> {
    pub track: Arc<Mutex<Track<T>>>,
    pub current_value: Arc<Mutex<T>>,
    pub current_velocity: Arc<Mutex<T>>,
    pub last_elapsed: Arc<Mutex<Duration>>,
}

impl<T: Debug> Debug for Prop<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cur = self.current_value.try_lock().ok().map(|v| format!("{:?}", *v));
        let vel = self.current_velocity.try_lock().ok().map(|v| format!("{:?}", *v));
        let elapsed = self.last_elapsed.try_lock().ok().map(|g| *g);

        f.debug_struct("Prop")
            .field("current_value", &cur)
            .field("current_velocity", &vel)
            .field("last_elapsed", &elapsed)
            .finish_non_exhaustive()
    }
}

impl<T: Clone + Interpolate + Send + Sync + Debug + 'static> Prop<T> {
    /// Constructs a new property starting from an initial value.
    pub fn new(initial: T) -> Self {
        Self {
            track: Arc::new(Mutex::new(Track::new(initial.clone()))),
            current_value: Arc::new(Mutex::new(initial.clone())),
            current_velocity: Arc::new(Mutex::new(initial.clone())),
            last_elapsed: Arc::new(Mutex::new(Duration::ZERO)),
        }
    }

    /// Constructs a new property from an existing `Track<T>`.
    pub fn from_track(track: Track<T>) -> Self {
        let initial = track.initial.clone();
        Self {
            track: Arc::new(Mutex::new(track)),
            current_value: Arc::new(Mutex::new(initial.clone())),
            current_velocity: Arc::new(Mutex::new(initial.clone())),
            last_elapsed: Arc::new(Mutex::new(Duration::ZERO)),
        }
    }

    /// Reads the current animated value at the latest sampled frame.
    pub fn get(&self) -> T {
        self.current_value.lock().unwrap().clone()
    }

    /// Samples the track at `elapsed` time and updates cached value and velocity derivatives.
    pub fn update(&self, elapsed: Duration) {
        let track = self.track.lock().unwrap();
        let (sampled_val, sampled_vel) = track.sample_state(elapsed);
        *self.current_value.lock().unwrap() = sampled_val;
        *self.current_velocity.lock().unwrap() = sampled_vel;
        *self.last_elapsed.lock().unwrap() = elapsed;
    }


    /// Returns the current instantaneous velocity derivative of the property.
    pub fn velocity(&self) -> T {
        self.current_velocity.lock().unwrap().clone()
    }

    /// Redirects a running animation mid-flight to a new target using an eased tween.
    pub fn interrupt_tween(&self, target: T, secs: f32, ease: Ease) -> &Self {
        let mut track = self.track.lock().unwrap();
        let last_elapsed = *self.last_elapsed.lock().unwrap();
        let (cur_val, cur_vel) = track.sample_state(last_elapsed);

        track.segments.clear();
        track.initial = cur_val.clone();
        track.time_offset = last_elapsed;
        track.loop_mode = LoopMode::Once;
        track.segments.push(Box::new(
            TweenSegment::new(cur_val.clone(), target, Duration::from_secs_f32(secs)).with_ease(ease),
        ));

        *self.current_value.lock().unwrap() = cur_val;
        *self.current_velocity.lock().unwrap() = cur_vel;
        self
    }
}

pub trait Animatable: Clone + Interpolate + Send + Sync + Debug + 'static {}
impl<T: Clone + Interpolate + Send + Sync + Debug + 'static> Animatable for T {}

impl<T: Animatable> Prop<T> {
    /// Appends a linear/eased tween segment.
    pub fn tween(&self, target: T, secs: f32) -> &Self {
        let mut track = self.track.lock().unwrap();
        let start = track.current_end_value();
        track.segments.push(Box::new(TweenSegment::new(
            start,
            target,
            Duration::from_secs_f32(secs),
        )));
        self
    }

    /// Modifies the easing profile of the most recently chained segment.
    pub fn ease(&self, ease: Ease) -> &Self {
        let mut track = self.track.lock().unwrap();
        if let Some(last) = track.segments.last_mut() {
            last.set_ease(ease);
        }
        self
    }

    /// Appends a tween segment with a specific easing profile in a single call.
    pub fn tween_eased(&self, target: T, secs: f32, ease: Ease) -> &Self {
        let mut track = self.track.lock().unwrap();
        let start = track.current_end_value();
        track.segments.push(Box::new(
            TweenSegment::new(start, target, Duration::from_secs_f32(secs)).with_ease(ease),
        ));
        self
    }

    // --- Temporal Constraints ---

    /// Scales the most recently appended segment so it completes within `secs`.
    pub fn fit_to_duration(&self, secs: f32) -> &Self {
        let mut track = self.track.lock().unwrap();
        if let Some(last) = track.segments.pop() {
            track.segments.push(Box::new(ConstrainedSegment::fit(
                last,
                Duration::from_secs_f32(secs),
            )));
        }
        self
    }

    /// Truncates the most recently appended segment at `secs`, snapping to rest once exceeded.
    pub fn clamp_at_duration(&self, secs: f32) -> &Self {
        let mut track = self.track.lock().unwrap();
        if let Some(last) = track.segments.pop() {
            track.segments.push(Box::new(ConstrainedSegment::clamp(
                last,
                Duration::from_secs_f32(secs),
            )));
        }
        self
    }

    // --- Delays and Choreography Holds ---

    /// Pauses at the current value for `secs` before following segments begin.
    pub fn hold(&self, secs: f32) -> &Self {
        let mut track = self.track.lock().unwrap();
        let start = track.current_end_value();
        track.segments.push(Box::new(HoldSegment::new(
            start,
            Duration::from_secs_f32(secs),
        )));
        self
    }

    /// Alias for `.hold()`, used at the start or middle of choreography chains.
    pub fn delay(&self, secs: f32) -> &Self {
        self.hold(secs)
    }

    // --- Loop Policies ---

    pub fn loop_mode(&self, mode: LoopMode) -> &Self {
        let mut track = self.track.lock().unwrap();
        track.loop_mode = mode;
        self
    }

    pub fn loop_forever(&self) -> &Self {
        self.loop_mode(LoopMode::LoopForever)
    }

    pub fn play_once(&self) -> &Self {
        self.loop_mode(LoopMode::Once)
    }

    pub fn loop_count(&self, count: usize) -> &Self {
        self.loop_mode(LoopMode::Count(count))
    }

    pub fn ping_pong(&self) -> &Self {
        self.loop_mode(LoopMode::PingPong)
    }

    pub fn ping_pong_count(&self, count: usize) -> &Self {
        self.loop_mode(LoopMode::PingPongCount(count))
    }

}

impl Prop<f32> {
    /// Appends an analytical harmonic spring segment.
    pub fn spring(&self, target: f32, params: SpringParams) -> &Self {
        let mut track = self.track.lock().unwrap();
        let start = track.current_end_value();
        track.segments.push(Box::new(SpringSegment::new(start, target, params)));
        self
    }

    /// Appends an analytical gravity bounce segment.
    pub fn gravity(&self, params: GravityParams, max_bounces: usize) -> &Self {
        let mut track = self.track.lock().unwrap();
        let start = track.current_end_value();
        track.segments.push(Box::new(GravitySegment::new(start, params, max_bounces)));
        self
    }

    /// Appends an analytical kinetic friction decay segment to the Prop.
    pub fn flick(&self, params: FlickParams) -> &Self {
        let mut track = self.track.lock().unwrap();
        let start = track.current_end_value();
        track.segments.push(Box::new(FlickSegment::new(start, params)));
        self
    }

    /// Redirects a running numeric animation mid-flight to a new target using an analytical spring.
    /// Preserves instantaneous velocity ($v_0 = \dot{x}_{\text{current}}$) for $C^1$ continuity.
    pub fn interrupt_spring(&self, target: f32, mut params: SpringParams) -> &Self {
        let mut track = self.track.lock().unwrap();
        let last_elapsed = *self.last_elapsed.lock().unwrap();
        let (cur_val, cur_vel) = track.sample_state(last_elapsed);

        // Inherit running momentum
        params.initial_velocity = cur_vel;

        track.segments.clear();
        track.initial = cur_val;
        track.time_offset = last_elapsed;
        track.loop_mode = LoopMode::Once;
        track.segments.push(Box::new(SpringSegment::new(cur_val, target, params)));

        *self.current_value.lock().unwrap() = cur_val;
        *self.current_velocity.lock().unwrap() = cur_vel;
        self
    }

    /// Interrupts a running animation with an inertial flick decay.
    /// If `params.initial_velocity == 0.0`, running velocity is inherited automatically.
    pub fn interrupt_flick(&self, mut params: FlickParams) -> &Self {
        let mut track = self.track.lock().unwrap();
        let last_elapsed = *self.last_elapsed.lock().unwrap();
        let (cur_val, cur_vel) = track.sample_state(last_elapsed);

        if params.initial_velocity == 0.0 {
            params.initial_velocity = cur_vel;
        }

        track.segments.clear();
        track.initial = cur_val;
        track.time_offset = last_elapsed;
        track.loop_mode = LoopMode::Once;
        track.segments.push(Box::new(FlickSegment::new(cur_val, params)));

        *self.current_value.lock().unwrap() = cur_val;
        *self.current_velocity.lock().unwrap() = cur_vel;
        self
    }

    /// Instantly halts movement at current position with zero velocity.
    pub fn stop(&self) -> &Self {
        let mut track = self.track.lock().unwrap();
        let last_elapsed = *self.last_elapsed.lock().unwrap();
        let cur_val = track.sample(last_elapsed);

        track.segments.clear();
        track.initial = cur_val;
        track.time_offset = last_elapsed;
        track.loop_mode = LoopMode::Once;

        *self.current_value.lock().unwrap() = cur_val;
        *self.current_velocity.lock().unwrap() = 0.0;
        self
    }
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

