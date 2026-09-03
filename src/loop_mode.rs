use std::time::Duration;
use std::fmt::Debug;

/// Playback policy governing how a Track cycles or terminates.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum LoopMode {
    #[default]
    LoopForever,
    Once,
    Count(usize),
    PingPong,
    PingPongCount(usize),
}

impl LoopMode {
    /// Transforms raw elapsed playback duration into local track time within `[0, total_duration]`.
    pub fn map_time(&self, elapsed: Duration, total_duration: Duration) -> Duration {
        if total_duration.is_zero() {
            return Duration::ZERO;
        }

        let t = elapsed.as_secs_f32();
        let total = total_duration.as_secs_f32();

        let local_secs = match self {
            LoopMode::Once => t.min(total),
            LoopMode::LoopForever => t % total,
            LoopMode::Count(count) => {
                if *count == 0 {
                    0.0
                } else if t >= (*count as f32) * total {
                    total
                } else {
                    t % total
                }
            }
            LoopMode::PingPong => {
                let cycle = 2.0 * total;
                let tau = t % cycle;
                if tau <= total {
                    tau
                } else {
                    cycle - tau
                }
            }
            LoopMode::PingPongCount(count) => {
                if *count == 0 {
                    0.0
                } else {
                    let k = (t / total).floor() as usize;
                    if k >= *count {
                        if *count % 2 == 0 {
                            0.0
                        } else {
                            total
                        }
                    } else {
                        let rem = t - (k as f32) * total;
                        if k % 2 == 0 {
                            rem
                        } else {
                            total - rem
                        }
                    }
                }
            }
        };

        Duration::from_secs_f32(local_secs.clamp(0.0, total))
    }
}
