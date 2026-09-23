use crate::timeline::master::MasterTimeline;
use gpui::*;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const SPEED_PRESETS: [f32; 4] = [0.5, 1.0, 1.5, 2.0];

/// Interactive transport bar and draggable timeline scrubber for `MasterTimeline`.
pub struct TimelineScrubber {
    pub timeline: Arc<Mutex<MasterTimeline>>,
    is_scrubbing: bool,
    was_playing_before_scrub: bool,
    rail_bounds: Arc<Mutex<(Pixels, Pixels)>>, // (origin_x, width)
}

impl TimelineScrubber {
    pub fn new(timeline: Arc<Mutex<MasterTimeline>>) -> Self {
        Self {
            timeline,
            is_scrubbing: false,
            was_playing_before_scrub: false,
            rail_bounds: Arc::new(Mutex::new((px(0.0), px(1.0)))),
        }
    }

    fn format_timecode(elapsed: Duration, total: Duration) -> String {
        let el_total_cs = elapsed.as_millis() / 10;
        let el_secs = (el_total_cs / 100) % 60;
        let el_mins = (el_total_cs / 6000) % 60;
        let el_cs = el_total_cs % 100;

        let tot_total_cs = total.as_millis() / 10;
        let tot_secs = (tot_total_cs / 100) % 60;
        let tot_mins = (tot_total_cs / 6000) % 60;
        let tot_cs = tot_total_cs % 100;

        format!(
            "{:02}:{:02}.{:02} / {:02}:{:02}.{:02}",
            el_mins, el_secs, el_cs, tot_mins, tot_secs, tot_cs
        )
    }

    fn cycle_speed(&self) {
        let mut tm = self.timeline.lock().unwrap();
        let cur = tm.playback_speed();
        let next_idx = SPEED_PRESETS
            .iter()
            .position(|&s| (s - cur).abs() < 0.05)
            .map(|i| (i + 1) % SPEED_PRESETS.len())
            .unwrap_or(1);
        tm.set_speed(SPEED_PRESETS[next_idx]);
    }
}

impl Render for TimelineScrubber {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let (is_playing, progress, playhead, total_duration, current_speed) = {
            let tm = self.timeline.lock().unwrap();
            (
                tm.is_playing(),
                tm.progress(),
                tm.playhead(),
                tm.duration(),
                tm.playback_speed(),
            )
        };

        let rail_bounds_ref = Arc::clone(&self.rail_bounds);
        let timecode_str = Self::format_timecode(playhead, total_duration);

        div()
            .w_full()
            .px_4()
            .py_2()
            .bg(hsla(0.65, 0.24, 0.10, 0.95))
            .border_t_1()
            .border_color(hsla(0.65, 0.20, 0.20, 0.8))
            .flex()
            .items_center()
            .gap_4()
            // -------------------------------------------------------------
            // 1. TRANSPORT BUTTONS (RESTART, PLAY/PAUSE, SPEED)
            // -------------------------------------------------------------
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    // Restart button
                    .child(
                        div()
                            .id("scrubber_restart")
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .bg(hsla(0.65, 0.18, 0.16, 1.0))
                            .hover(|s| s.bg(hsla(0.65, 0.25, 0.24, 1.0)))
                            .active(|s| s.bg(hsla(0.65, 0.30, 0.28, 1.0)))
                            .cursor_pointer()
                            .text_xs()
                            .text_color(hsla(0.0, 0.0, 0.90, 1.0))
                            .child("⏮")
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.timeline.lock().unwrap().restart();
                                    cx.notify();
                                }),
                            ),
                    )
                    // Play / Pause button
                    .child(
                        div()
                            .id("scrubber_play_pause")
                            .w(px(32.0))
                            .h(px(26.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded_md()
                            .bg(if is_playing {
                                hsla(0.65, 0.20, 0.18, 1.0)
                            } else {
                                hsla(0.52, 0.85, 0.40, 0.85)
                            })
                            .hover(|s| s.bg(hsla(0.52, 0.85, 0.48, 1.0)))
                            .cursor_pointer()
                            .text_xs()
                            .text_color(hsla(0.0, 0.0, 0.98, 1.0))
                            .child(if is_playing { "⏸" } else { "▶" })
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.timeline.lock().unwrap().toggle();
                                    cx.notify();
                                }),
                            ),
                    )
                    // Playback speed cycle pill
                    .child(
                        div()
                            .id("scrubber_speed")
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .bg(hsla(0.65, 0.18, 0.16, 1.0))
                            .hover(|s| s.bg(hsla(0.65, 0.25, 0.24, 1.0)))
                            .cursor_pointer()
                            .text_xs()
                            .font_family(".SystemUIFont")
                            .text_color(hsla(0.52, 0.90, 0.65, 1.0))
                            .child(format!("{:.1}x", current_speed))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    this.cycle_speed();
                                    cx.notify();
                                }),
                            ),
                    ),
            )
            // -------------------------------------------------------------
            // 2. INTERACTIVE DRAGGABLE SCRUBBER RAIL
            // -------------------------------------------------------------
            .child(
                div()
                    .id("scrubber_rail_wrapper")
                    .relative()
                    .flex_1()
                    .h(px(28.0))
                    .flex()
                    .items_center()
                    .cursor_pointer()
                    // Drag and Scrub Event Handlers
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, event: &MouseDownEvent, _, cx| {
                            let (orig_x, width) = *this.rail_bounds.lock().unwrap();
                            let total_w = f32::from(width).max(1.0);
                            let local_x = f32::from(event.position.x - orig_x);
                            let target_progress = (local_x / total_w).clamp(0.0, 1.0);

                            let was_playing = this.timeline.lock().unwrap().is_playing();
                            this.was_playing_before_scrub = was_playing;
                            this.is_scrubbing = true;

                            let mut tm = this.timeline.lock().unwrap();
                            tm.pause();
                            tm.seek_progress(target_progress);
                            cx.notify();
                        }),
                    )
                    .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _, cx| {
                        if this.is_scrubbing {
                            let (orig_x, width) = *this.rail_bounds.lock().unwrap();
                            let total_w = f32::from(width).max(1.0);
                            let local_x = f32::from(event.position.x - orig_x);
                            let target_progress = (local_x / total_w).clamp(0.0, 1.0);

                            this.timeline.lock().unwrap().seek_progress(target_progress);
                            cx.notify();
                        }
                    }))
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| {
                            if this.is_scrubbing {
                                this.is_scrubbing = false;
                                if this.was_playing_before_scrub {
                                    this.timeline.lock().unwrap().play();
                                }
                                cx.notify();
                            }
                        }),
                    )
                    // Custom Canvas Layer: Background rail, tick marks, progress fill, and thumb needle
                    .child(
                        canvas(
                            |_, _, _| {},
                            move |bounds, _, window, _| {
                                *rail_bounds_ref.lock().unwrap() =
                                    (bounds.origin.x, bounds.size.width);

                                let rail_h = 6.0;
                                let center_y = bounds.origin.y + bounds.size.height / 2.0;
                                let rail_y = center_y - px(rail_h / 2.0);
                                let total_w = f32::from(bounds.size.width);

                                // Background track
                                window.paint_quad(
                                    fill(
                                        Bounds {
                                            origin: point(bounds.origin.x, rail_y),
                                            size: size(bounds.size.width, px(rail_h)),
                                        },
                                        hsla(0.65, 0.18, 0.20, 0.8),
                                    )
                                    .corner_radii(Corners::all(px(rail_h / 2.0))),
                                );

                                // Periodic tick rulers (25%, 50%, 75%)
                                for tick in [0.25, 0.50, 0.75] {
                                    let tick_x = bounds.origin.x + px(total_w * tick);
                                    window.paint_quad(fill(
                                        Bounds {
                                            origin: point(tick_x - px(0.5), center_y - px(6.0)),
                                            size: size(px(1.0), px(12.0)),
                                        },
                                        hsla(0.65, 0.15, 0.35, 0.5),
                                    ));
                                }

                                // Elapsed progress bar
                                let fill_w = (total_w * progress).clamp(0.0, total_w);
                                if fill_w > 0.0 {
                                    window.paint_quad(
                                        fill(
                                            Bounds {
                                                origin: point(bounds.origin.x, rail_y),
                                                size: size(px(fill_w), px(rail_h)),
                                            },
                                            hsla(0.52, 0.95, 0.55, 1.0),
                                        )
                                        .corner_radii(Corners::all(px(rail_h / 2.0))),
                                    );
                                }

                                // Playhead Thumb Needle
                                let thumb_x = bounds.origin.x + px(fill_w) - px(7.0);
                                let thumb_y = center_y - px(7.0);

                                // Thumb outer glow / drop shadow
                                window.paint_quad(
                                    fill(
                                        Bounds {
                                            origin: point(thumb_x - px(2.0), thumb_y - px(2.0)),
                                            size: size(px(18.0), px(18.0)),
                                        },
                                        hsla(0.52, 0.95, 0.55, 0.25),
                                    )
                                    .corner_radii(Corners::all(px(9.0))),
                                );

                                // Thumb white core
                                window.paint_quad(
                                    fill(
                                        Bounds {
                                            origin: point(thumb_x, thumb_y),
                                            size: size(px(14.0), px(14.0)),
                                        },
                                        hsla(0.0, 0.0, 1.0, 1.0),
                                    )
                                    .corner_radii(Corners::all(px(7.0))),
                                );
                            },
                        )
                        .size_full(),
                    ),
            )
            // -------------------------------------------------------------
            // 3. TIMECODE DIGITAL READOUT
            // -------------------------------------------------------------
            .child(
                div()
                    .min_w(px(140.0))
                    .flex()
                    .justify_end()
                    .text_xs()
                    .font_family(".SystemUIFont")
                    .text_color(hsla(0.65, 0.15, 0.65, 1.0))
                    .child(timecode_str),
            )
    }
}
