use crate::interpolate::Interpolate;
use crate::property::Prop;
use crate::timeline::track_binding::{BoundTrack, TimelineTrack};
use crate::loop_mode::LoopMode;
use std::collections::HashMap;
use std::fmt::Debug;
use std::time::Duration;
use crate::IntoSeqGroup;

/// Global coordinator orchestrating multi-track timelines, cue points, and scrubbing.
pub struct MasterTimeline {
    tracks: Vec<Box<dyn TimelineTrack>>,
    cues: HashMap<String, Duration>,
    playhead: Duration,
    is_playing: bool,
    playback_speed: f32,
    loop_mode: LoopMode,
}

impl Debug for MasterTimeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MasterTimeline")
            .field("tracks_count", &self.tracks.len())
            .field("cues", &self.cues)
            .field("playhead", &self.playhead)
            .field("is_playing", &self.is_playing)
            .field("playback_speed", &self.playback_speed)
            .field("loop_mode", &self.loop_mode)
            .finish()
    }
}

impl Default for MasterTimeline {
    fn default() -> Self {
        Self::new()
    }
}

impl MasterTimeline {
    pub fn new() -> Self {
        Self {
            tracks: Vec::new(),
            cues: HashMap::new(),
            playhead: Duration::ZERO,
            is_playing: true,
            playback_speed: 1.0,
            loop_mode: LoopMode::LoopForever,
        }
    }

    pub fn loop_mode(mut self, mode: LoopMode) -> Self {
        self.loop_mode = mode;
        self
    }

    pub fn loop_forever(self) -> Self {
        self.loop_mode(LoopMode::LoopForever)
    }

    pub fn play_once(self) -> Self {
        self.loop_mode(LoopMode::Once)
    }

    /// Appends a sequence of tracks scheduled back-to-back, starting at the current end of the timeline.
    pub fn seq(mut self, group: impl IntoSeqGroup) -> Self {
        let start_offset = self.duration();
        group.add_to_timeline(start_offset, &mut self);
        self
    }

    /// Appends a sequence of tracks scheduled back-to-back starting at an explicit timestamp `start_secs`.
    pub fn seq_at(mut self, start_secs: f32, group: impl IntoSeqGroup) -> Self {
        let start_offset = Duration::from_secs_f32(start_secs.max(0.0));
        group.add_to_timeline(start_offset, &mut self);
        self
    }

    /// Appends a sequence of tracks starting immediately after a named cue point or completed track.
    pub fn seq_after(mut self, target_name: &str, group: impl IntoSeqGroup) -> Self {
        let start_offset = self.resolve_start_time(target_name);
        group.add_to_timeline(start_offset, &mut self);
        self
    }

    // --- Track Registration & DSL ---

    /// Registers a custom pre-boxed `TimelineTrack`.
    pub fn add_track(&mut self, track: Box<dyn TimelineTrack>) -> &mut Self {
        self.tracks.push(track);
        self
    }

    /// Attaches an unnamed property track scheduled to begin at `offset_secs`.
    pub fn at<T: Clone + Interpolate + Send + Sync + Debug + 'static>(
        mut self,
        offset_secs: f32,
        prop: Prop<T>,
    ) -> Self {
        let offset = Duration::from_secs_f32(offset_secs.max(0.0));
        self.tracks
            .push(Box::new(BoundTrack::new(None, offset, prop)));
        self
    }

    /// Attaches a named property track scheduled to begin at `offset_secs`.
    pub fn at_named<T: Clone + Interpolate + Send + Sync + Debug + 'static>(
        mut self,
        name: impl Into<String>,
        offset_secs: f32,
        prop: Prop<T>,
    ) -> Self {
        let offset = Duration::from_secs_f32(offset_secs.max(0.0));
        self.tracks
            .push(Box::new(BoundTrack::new(Some(name.into()), offset, prop)));
        self
    }

    /// Defines a named timestamp cue point along the timeline.
    pub fn cue(mut self, name: impl Into<String>, timestamp_secs: f32) -> Self {
        self.cues.insert(
            name.into(),
            Duration::from_secs_f32(timestamp_secs.max(0.0)),
        );
        self
    }

    /// Schedules a property to start immediately when a cue point fires,
    /// or when a named track finishes its duration.
    pub fn after<T: Clone + Interpolate + Send + Sync + Debug + 'static>(
        mut self,
        target_name: &str,
        prop: Prop<T>,
    ) -> Self {
        let start_offset = self.resolve_start_time(target_name);
        self.tracks
            .push(Box::new(BoundTrack::new(None, start_offset, prop)));
        self
    }

    /// Schedules a named property track to start immediately after a cue point or named track.
    pub fn after_named<T: Clone + Interpolate + Send + Sync + Debug + 'static>(
        mut self,
        name: impl Into<String>,
        target_name: &str,
        prop: Prop<T>,
    ) -> Self {
        let start_offset = self.resolve_start_time(target_name);
        self.tracks.push(Box::new(BoundTrack::new(
            Some(name.into()),
            start_offset,
            prop,
        )));
        self
    }

    fn resolve_start_time(&self, target_name: &str) -> Duration {
        if let Some(&cue_time) = self.cues.get(target_name) {
            return cue_time;
        }

        for track in &self.tracks {
            if track.name() == Some(target_name) {
                return track.duration();
            }
        }

        // Fallback to start of timeline if cue or track not found
        Duration::ZERO
    }

    // --- Playback & Scrubbing ---

    /// Evaluates the master timeline and all child tracks at timestamp `t`.
    pub fn seek(&mut self, t: Duration) {
        let total = self.duration();
        let mapped = self.loop_mode.map_time(t, total);
        self.playhead = mapped;

        for track in &self.tracks {
            track.seek(mapped);
        }
    }

    /// Seeks normalized timeline progress `progress ∈ [0.0, 1.0]`.
    pub fn seek_progress(&mut self, progress: f32) {
        let total = self.duration();
        let target_secs = progress.clamp(0.0, 1.0) * total.as_secs_f32();
        self.seek(Duration::from_secs_f32(target_secs));
    }

    /// Advances the timeline clock by real time `dt`, scaled by `playback_speed`.
    pub fn advance(&mut self, dt: Duration) {
        if !self.is_playing || dt.is_zero() {
            return;
        }

        let scaled_secs = dt.as_secs_f32() * self.playback_speed;
        let total = self.duration();
        if total.is_zero() {
            return;
        }

        let next_unmapped = self.playhead + Duration::from_secs_f32(scaled_secs);
        let mapped = self.loop_mode.map_time(next_unmapped, total);
        self.playhead = mapped;

        for track in &self.tracks {
            track.seek(mapped);
        }
    }

    // --- Transport Controls ---

    pub fn play(&mut self) {
        self.is_playing = true;
    }

    pub fn pause(&mut self) {
        self.is_playing = false;
    }

    pub fn toggle(&mut self) {
        self.is_playing = !self.is_playing;
    }

    pub fn restart(&mut self) {
        self.seek(Duration::ZERO);
        self.is_playing = true;
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.playback_speed = speed.max(0.0);
    }

    pub fn set_loop_mode(&mut self, mode: LoopMode) {
        self.loop_mode = mode;
    }

    // --- Queries ---

    /// Returns the maximum duration across all child tracks and cues.
    pub fn duration(&self) -> Duration {
        let max_track_dur = self
            .tracks
            .iter()
            .map(|t| t.duration())
            .max()
            .unwrap_or(Duration::ZERO);

        let max_cue_dur = self
            .cues
            .values()
            .copied()
            .max()
            .unwrap_or(Duration::ZERO);

        max_track_dur.max(max_cue_dur)
    }

    pub fn playhead(&self) -> Duration {
        self.playhead
    }

    pub fn progress(&self) -> f32 {
        let total = self.duration().as_secs_f32();
        if total <= 0.0 {
            0.0
        } else {
            (self.playhead.as_secs_f32() / total).clamp(0.0, 1.0)
        }
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    pub fn playback_speed(&self) -> f32 {
        self.playback_speed
    }
}

/// Standalone builder creating a new `MasterTimeline` from a sequential series.
pub fn seq(group: impl IntoSeqGroup) -> MasterTimeline {
    MasterTimeline::new().seq(group)
}
