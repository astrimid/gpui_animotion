mod shared;

use gpui::*;
use gpui_animotion::*;
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

const SPEED_PRESETS: [f32; 4] = [0.5, 1.0, 1.5, 2.0];

// =========================================================================
// 1. ENGINE ELEMENT: VSYNC-DRIVEN TIMELINE WRAPPER
// =========================================================================

pub struct AnimotionTimelineElement {
    id: ElementId,
    timeline: Arc<Mutex<MasterTimeline>>,
    child: Option<AnyElement>,
}

impl AnimotionTimelineElement {
    pub fn new(id: impl Into<ElementId>, timeline: Arc<Mutex<MasterTimeline>>, child: impl IntoElement) -> Self {
        Self {
            id: id.into(),
            timeline,
            child: Some(child.into_any_element()),
        }
    }
}

impl IntoElement for AnimotionTimelineElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for AnimotionTimelineElement {
    type RequestLayoutState = AnyElement;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let global_id = id.unwrap();

        let (layout_id, any_el) = window.with_element_state(
            global_id,
            |state: Option<Instant>, window_cx| {
                let now = Instant::now();
                let last = state.unwrap_or(now);
                let dt = now.duration_since(last);

                // Advance timeline playhead on hardware VSync tick
                let is_playing = {
                    let mut tm = self.timeline.lock().unwrap();
                    if tm.is_playing() {
                        tm.advance(dt);
                        true
                    } else {
                        false
                    }
                };

                // Request next frame ONLY while timeline is playing
                if is_playing {
                    window_cx.request_animation_frame();
                }

                let mut child = self.child.take().unwrap();
                let layout_id = child.request_layout(window_cx, cx);

                ((layout_id, child), now)
            },
        );

        (layout_id, any_el)
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        element: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) {
        element.prepaint(window, cx);
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        element: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        element.paint(window, cx);
    }
}

pub trait AnimotionTimelineExt: IntoElement + Sized {
    /// Binds this element tree to a hardware VSync-driven MasterTimeline.
    fn animotion_timeline(
        self,
        id: impl Into<ElementId>,
        timeline: Arc<Mutex<MasterTimeline>>,
    ) -> AnimotionTimelineElement {
        AnimotionTimelineElement::new(id, timeline, self)
    }
}

impl<E: IntoElement> AnimotionTimelineExt for E {}

// =========================================================================
// 2. COMPONENT: APP HEADER
// =========================================================================

fn studio_header(is_looping: bool) -> impl IntoElement {
    div()
        .w_full()
        .h(px(44.0))
        .px_6()
        .bg(hsla(0.65, 0.25, 0.11, 0.95))
        .border_b_1()
        .border_color(hsla(0.65, 0.20, 0.20, 0.8))
        .flex()
        .items_center()
        .justify_between()
        .child(
            div()
                .flex()
                .items_center()
                .gap_3()
                .child(
                    div()
                        .size_3()
                        .rounded_full()
                        .bg(hsla(0.48, 0.95, 0.55, 1.0)),
                )
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::BOLD)
                        .text_color(hsla(0.0, 0.0, 0.95, 1.0))
                        .child("Animotion Studio"),
                )
                .child(
                    div()
                        .px_2()
                        .py_0p5()
                        .rounded_md()
                        .bg(hsla(0.65, 0.20, 0.18, 1.0))
                        .text_xs()
                        .text_color(hsla(0.65, 0.15, 0.65, 1.0))
                        .child("Stage Viewport"),
                ),
        )
        .child(
            div()
                .text_xs()
                .text_color(hsla(0.65, 0.15, 0.55, 1.0))
                .child(if is_looping {
                    "Loop: Forever | Deterministic: O(1)"
                } else {
                    "Loop: Once | Deterministic: O(1)"
                }),
        )
}

// =========================================================================
// 3. COMPONENT: RADAR BACKGROUND CANVAS
// =========================================================================

// In examples/master_timeline_studio.rs

fn radar_background(dial_angle: f32) -> impl IntoElement {
    canvas(
        |_, _, _| {},
        move |bounds, _, window, _| {
            let center_x = bounds.origin.x + bounds.size.width / 2.0;
            let center_y = bounds.origin.y + bounds.size.height / 2.0;

            // Constrain radius to stage viewport height with a 24px safety margin
            let half_h = f32::from(bounds.size.height / 2.0 - px(24.0));
            let half_w = f32::from(bounds.size.width / 2.0 - px(24.0));
            let max_r = half_h.min(half_w).max(80.0);

            let r1 = max_r * 0.40;
            let r2 = max_r * 0.70;
            let r3 = max_r * 0.98;

            for radius in [r1, r2, r3] {
                let mut ring_builder = PathBuilder::fill();
                for i in 0..48 {
                    let theta = (i as f32 / 48.0) * 2.0 * PI;
                    let pt = point(
                        center_x + px(radius * theta.cos()),
                        center_y + px(radius * theta.sin()),
                    );
                    if i == 0 {
                        ring_builder.move_to(pt);
                    } else {
                        ring_builder.line_to(pt);
                    }
                }
                if let Ok(path) = ring_builder.build() {
                    window.paint_path(path, hsla(0.65, 0.20, 0.25, 0.08));
                }
            }

            // Orbiting satellite rides along the middle ring (r2)
            let rad_theta = dial_angle * PI / 180.0;
            let sat_x = center_x + px(r2 * rad_theta.cos());
            let sat_y = center_y + px(r2 * rad_theta.sin());

            window.paint_quad(
                fill(
                    Bounds {
                        origin: point(sat_x - px(5.0), sat_y - px(5.0)),
                        size: size(px(10.0), px(10.0)),
                    },
                    hsla(0.48, 0.95, 0.60, 0.75),
                )
                .corner_radii(Corners::all(px(5.0))),
            );
        },
    )
    .absolute()
    .inset_0()
    .size_full()
}

// =========================================================================
// 4. COMPONENT: HERO GLASS CARD
// =========================================================================

struct HeroCardProps {
    pub y_offset: f32,
    pub opacity: f32,
    pub scale: f32,
    pub glow: Hsla,
    pub progress: f32,
    pub angle: f32,
}

fn hero_card(props: HeroCardProps) -> impl IntoElement {
    let badge_scale = props.scale.clamp(0.0, 1.5);

    div()
        .relative()
        .top(px(props.y_offset))
        .opacity(props.opacity)
        .w(px(520.0))
        .h(px(260.0))
        .rounded_2xl()
        .border_1()
        .border_color(props.glow)
        .bg(hsla(0.65, 0.24, 0.12, 0.94))
        .shadow_xl()
        .overflow_hidden()
        .flex()
        .flex_col()
        .justify_between()
        .child(
            div()
                .h(px(4.0))
                .w(px(props.progress.min(520.0)))
                .bg(props.glow),
        )
        .child(
            div()
                .p_6()
                .flex()
                .flex_col()
                .gap_3()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_lg()
                                .font_weight(FontWeight::BOLD)
                                .text_color(hsla(0.0, 0.0, 0.98, 1.0))
                                .child("Cluster Node 01"),
                        )
                        .child(
                            div()
                                .h(px(22.0 * badge_scale))
                                .px(px(12.0 * badge_scale))
                                .py(px(2.0 * badge_scale))
                                .rounded_full()
                                .bg(hsla(0.38, 0.85, 0.45, 0.25 * badge_scale.min(1.0)))
                                .border_1()
                                .border_color(hsla(0.38, 0.90, 0.55, 0.8 * badge_scale.min(1.0)))
                                .overflow_hidden()
                                .flex()
                                .items_center()
                                .text_xs()
                                .text_color(hsla(0.38, 0.90, 0.70, badge_scale.min(1.0)))
                                .child("● HEALTHY"),
                        ),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(hsla(0.65, 0.15, 0.65, 1.0))
                        .child("Deterministic procedural orchestration combining analytical springs and timeline synchronization."),
                ),
        )
        .child(
            div()
                .px_6()
                .py_4()
                .bg(hsla(0.65, 0.22, 0.09, 0.80))
                .border_t_1()
                .border_color(hsla(0.65, 0.20, 0.18, 0.6))
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .text_xs()
                        .text_color(hsla(0.65, 0.15, 0.55, 1.0))
                        .child(format!("Dial Position: {:.0}°", props.angle)),
                )
                .child(
                    div()
                        .px_3()
                        .py_1p5()
                        .rounded_lg()
                        .bg(props.glow)
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(hsla(0.65, 0.30, 0.08, 1.0))
                        .child("Inspect Properties"),
                ),
        )
}

// =========================================================================
// 5. ROOT VIEW & COORDINATOR
// =========================================================================

struct MasterTimelineStudioView {
    timeline: Arc<Mutex<MasterTimeline>>,
    rail_bounds: Arc<Mutex<(Pixels, Pixels)>>,
    is_scrubbing: bool,
    was_playing_before_scrub: bool,
    hero_y: Prop<f32>,
    hero_opacity: Prop<f32>,
    badge_scale: Prop<f32>,
    glow_color: Prop<Hsla>,
    progress_bar: Prop<f32>,
    dial_angle: Prop<f32>,
}

impl MasterTimelineStudioView {
    pub fn new() -> Self {
        let hero_y = Prop::new(-150.0);
        hero_y
            .spring(0.0, SpringParams::from_damping_ratio(160.0, 0.42))
            .fit_to_duration(0.70)
            .play_once();

        // Fade in from 0% to 100% opacity in 0.35s
        let hero_opacity = Prop::new(0.0);
        hero_opacity.tween_eased(1.0, 0.35, Ease::OutQuad)
            .play_once();

        let badge_scale = Prop::new(0.0);
        badge_scale
            .spring(1.0, SpringParams::from_damping_ratio(220.0, 0.32))
            .fit_to_duration(0.45)
            .play_once();

        let glow_color = Prop::new(hsla(0.55, 0.20, 0.20, 0.0));
        glow_color.tween_eased(hsla(0.48, 0.95, 0.55, 0.85), 0.55, Ease::OutCubic).play_once();

        let progress_bar = Prop::new(0.0);
        progress_bar.tween_eased(640.0, 1.70, Ease::Linear).play_once();

        let dial_angle = Prop::new(0.0);
        dial_angle.tween_eased(360.0, 1.70, Ease::InOutCubic).play_once();

        let timeline = Arc::new(Mutex::new(
            MasterTimeline::new()
                .at(0.0, progress_bar.clone())
                .at(0.0, dial_angle.clone())
                .at(0.0, hero_opacity.clone())
                .seq((
                    ("card_entry", hero_y.clone()),
                    ("badge_pop", badge_scale.clone()),
                    ("glow_activate", glow_color.clone()),
                ))
                .cue("scene_loop", 3.20)
                .loop_forever(),
        ));

        Self {
            timeline,
            rail_bounds: Arc::new(Mutex::new((px(0.0), px(1.0)))),
            is_scrubbing: false,
            was_playing_before_scrub: false,
            hero_y,
            hero_opacity,
            badge_scale,
            glow_color,
            progress_bar,
            dial_angle,
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

    fn render_scrubber(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
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
            // 1. Transport Controls
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
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
                            .text_color(hsla(0.52, 0.90, 0.65, 1.0))
                            .child(format!("{:.1}x", current_speed))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, _, cx| {
                                    let mut tm = this.timeline.lock().unwrap();
                                    let cur = tm.playback_speed();
                                    let next_idx = SPEED_PRESETS
                                        .iter()
                                        .position(|&s| (s - cur).abs() < 0.05)
                                        .map(|i| (i + 1) % SPEED_PRESETS.len())
                                        .unwrap_or(1);
                                    tm.set_speed(SPEED_PRESETS[next_idx]);
                                    cx.notify();
                                }),
                            ),
                    ),
            )
            // 2. Interactive Scrubber Rail
            .child(
                div()
                    .id("scrubber_rail_wrapper")
                    .relative()
                    .flex_1()
                    .h(px(28.0))
                    .flex()
                    .items_center()
                    .cursor_pointer()
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

                                // Rail background
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

                                // Periodic tick marks (25%, 50%, 75%)
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

                                // Active progress fill
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

                                // Thumb needle
                                let thumb_x = bounds.origin.x + px(fill_w) - px(7.0);
                                let thumb_y = center_y - px(7.0);

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
            // 3. Timecode Readout
            .child(
                div()
                    .min_w(px(140.0))
                    .flex()
                    .justify_end()
                    .text_xs()
                    .text_color(hsla(0.65, 0.15, 0.65, 1.0))
                    .child(timecode_str),
            )
    }
}

impl Render for MasterTimelineStudioView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let card_y = self.hero_y.get();
        let card_opacity = self.hero_opacity.get();
        let badge_scale = self.badge_scale.get();
        let glow = self.glow_color.get();
        let progress = self.progress_bar.get();
        let angle = self.dial_angle.get();

        div()
            .size_full()
            .bg(hsla(0.65, 0.30, 0.07, 1.0))
            .flex()
            .flex_col()
            .overflow_hidden()
            .child(studio_header(true))
            .child(
                div()
                    .flex_1()
                    .relative()
                    .flex()
                    .items_center()
                    .justify_center()
                    .overflow_hidden()
                    .child(radar_background(angle))
                    .child(hero_card(HeroCardProps {
                        y_offset: card_y,
                        opacity: card_opacity,
                        scale: badge_scale,
                        glow,
                        progress,
                        angle,
                    })),
            )
            .child(self.render_scrubber(cx))
            .animotion_timeline("studio_timeline", self.timeline.clone())
    }
}

fn main() {
    shared::run("Master Timeline Studio & Scrubber", |_, _| {
        MasterTimelineStudioView::new()
    });
}
