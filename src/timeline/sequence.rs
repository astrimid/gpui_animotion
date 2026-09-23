use crate::interpolate::Interpolate;
use crate::property::Prop;
use crate::timeline::master::MasterTimeline;
use crate::timeline::track_binding::{BoundTrack, TimelineTrack};
use std::fmt::Debug;
use std::time::Duration;

/// An item that can be placed into a sequential timeline queue.
pub trait IntoSeqItem {
    /// Returns the execution duration of the item itself.
    fn item_duration(&self) -> Duration;

    /// Consumes the item and boxes it as a `TimelineTrack` starting at `start_offset`.
    fn into_timeline_track(self, start_offset: Duration) -> Box<dyn TimelineTrack>;
}

impl<T: Clone + Interpolate + Send + Sync + Debug + 'static> IntoSeqItem for Prop<T> {
    fn item_duration(&self) -> Duration {
        self.track.lock().unwrap().total_duration()
    }

    fn into_timeline_track(self, start_offset: Duration) -> Box<dyn TimelineTrack> {
        Box::new(BoundTrack::new(None, start_offset, self))
    }
}

impl<T: Clone + Interpolate + Send + Sync + Debug + 'static> IntoSeqItem for (&'static str, Prop<T>) {
    fn item_duration(&self) -> Duration {
        self.1.track.lock().unwrap().total_duration()
    }

    fn into_timeline_track(self, start_offset: Duration) -> Box<dyn TimelineTrack> {
        Box::new(BoundTrack::new(Some(self.0.to_string()), start_offset, self.1))
    }
}

impl<T: Clone + Interpolate + Send + Sync + Debug + 'static> IntoSeqItem for (String, Prop<T>) {
    fn item_duration(&self) -> Duration {
        self.1.track.lock().unwrap().total_duration()
    }

    fn into_timeline_track(self, start_offset: Duration) -> Box<dyn TimelineTrack> {
        Box::new(BoundTrack::new(Some(self.0), start_offset, self.1))
    }
}

/// A collection of items scheduled back-to-back along the timeline.
pub trait IntoSeqGroup {
    /// Schedules items sequentially starting from `start_time` and returns the final end timestamp.
    fn add_to_timeline(self, start_time: Duration, timeline: &mut MasterTimeline) -> Duration;
}

// Dynamic Vector support
impl<I: IntoSeqItem> IntoSeqGroup for Vec<I> {
    fn add_to_timeline(self, start_time: Duration, timeline: &mut MasterTimeline) -> Duration {
        let mut cursor = start_time;
        for item in self {
            let dur = item.item_duration();
            timeline.add_track(item.into_timeline_track(cursor));
            cursor += dur;
        }
        cursor
    }
}

// Homogeneous fixed-size array support
macro_rules! impl_into_seq_group_array {
    ($($N:expr),*) => {$(
            impl<I: IntoSeqItem> IntoSeqGroup for [I; $N] {
                fn add_to_timeline(self, start_time: Duration, timeline: &mut MasterTimeline) -> Duration {
                    let mut cursor = start_time;
                    for item in self {
                        let dur = item.item_duration();
                        timeline.add_track(item.into_timeline_track(cursor));
                        cursor += dur;
                    }
                    cursor
                }
            }
        )*
    };
}

impl_into_seq_group_array!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 16);

// Heterogeneous tuple support (different property types chained together)
macro_rules! impl_into_seq_group_tuple {
    ($($name:ident),+) => {
        impl<$($name: IntoSeqItem),+> IntoSeqGroup for ($($name,)+) {
            #[allow(non_snake_case)]
            fn add_to_timeline(self, start_time: Duration, timeline: &mut MasterTimeline) -> Duration {
                let ($($name,)+) = self;
                let mut cursor = start_time;
                $(
                    let dur = $name.item_duration();
                    timeline.add_track($name.into_timeline_track(cursor));
                    cursor += dur;
                )+
                cursor
            }
        }
    };
}

impl_into_seq_group_tuple!(A, B);
impl_into_seq_group_tuple!(A, B, C);
impl_into_seq_group_tuple!(A, B, C, D);
impl_into_seq_group_tuple!(A, B, C, D, E);
impl_into_seq_group_tuple!(A, B, C, D, E, F);
impl_into_seq_group_tuple!(A, B, C, D, E, F, G);
impl_into_seq_group_tuple!(A, B, C, D, E, F, G, H);
