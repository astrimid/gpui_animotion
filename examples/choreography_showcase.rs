mod shared;

use gpui::*;
use gpui_animotion::*;
use std::f32::consts::PI;

#[derive(Clone)]
struct ChoreographyRacerProps {
    x: Prop<f32>,
    color: Prop<Hsla>,
    title: &'static str,
    feature: &'static str,
    y: f32,
}

struct ChoreographyShowcaseView;

impl Render for ChoreographyShowcaseView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let track_start = 240.0;
        let track_end = 680.0;
        let marble_size = 36.0;

        div()
            .size_full()
            .bg(hsla(0.65, 0.28, 0.08, 1.0)) // Deep twilight background
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .child(
                div()
                    .relative()
                    .w(px(760.0))
                    .h(px(520.0))
                    .bg(hsla(0.65, 0.22, 0.12, 0.96))
                    .border_1()
                    .border_color(hsla(0.65, 0.25, 0.24, 0.8))
                    .rounded_2xl()
                    .shadow_xl()
                    .animotion_clip(
                        ElementId::Name("choreography_showcase_clip".into()),
                        |c| vec![
                            // -------------------------------------------------------------
                            // LANE 1: Natural Underdamped Spring (Unconstrained ~1.85s natural decay)
                            // -------------------------------------------------------------
                            ChoreographyRacerProps {
                                title: "1. Natural Spring",
                                feature: "Unconstrained (~1.85s settle)",
                                y: 40.0,
                                x: c.prop(track_start)
                                    .spring(track_end, SpringParams::from_damping_ratio(120.0, 0.20))
                                    .tween_eased(track_start, 0.8, Ease::OutCubic)
                                    .clone(),
                                color: c.prop(hsla(0.52, 0.90, 0.55, 1.0)),
                            },
                            // -------------------------------------------------------------
                            // LANE 2: Fitted Duration (Identical spring time-scaled into exactly 750ms)
                            // -------------------------------------------------------------
                            ChoreographyRacerProps {
                                title: "2. Fitted Duration",
                                feature: ".fit_to_duration(0.75s) (2.5x speed)",
                                y: 130.0,
                                x: c.prop(track_start)
                                    .spring(track_end, SpringParams::from_damping_ratio(120.0, 0.20))
                                    .fit_to_duration(0.75)
                                    .hold(0.35)
                                    .tween_eased(track_start, 0.5, Ease::OutCubic)
                                    .clone(),
                                color: c.prop(hsla(0.10, 0.95, 0.58, 1.0)),
                            },
                            // -------------------------------------------------------------
                            // LANE 3: Clamped Duration (Hard-cuts tail at 400ms, snaps to equilibrium)
                            // -------------------------------------------------------------
                            ChoreographyRacerProps {
                                title: "3. Clamped Duration",
                                feature: ".clamp_at_duration(0.40s) (hard cap)",
                                y: 220.0,
                                x: c.prop(track_start)
                                    .spring(track_end, SpringParams::from_damping_ratio(120.0, 0.20))
                                    .clamp_at_duration(0.40)
                                    .hold(0.60)
                                    .tween_eased(track_start, 0.6, Ease::InQuad)
                                    .clone(),
                                color: c.prop(hsla(0.98, 0.90, 0.60, 1.0)),
                            },
                            // -------------------------------------------------------------
                            // LANE 4: Ping-Pong Cycle (Automatic forward/reverse playback mirroring)
                            // -------------------------------------------------------------
                            ChoreographyRacerProps {
                                title: "4. Ping-Pong Cycle",
                                feature: ".ping_pong() (auto reverse pass)",
                                y: 310.0,
                                x: c.prop(track_start)
                                    .spring(track_end, SpringParams::from_damping_ratio(180.0, 0.35))
                                    .fit_to_duration(0.80)
                                    .hold(0.25)
                                    .ping_pong()
                                    .clone(),
                                color: c.prop(hsla(0.38, 0.90, 0.50, 1.0)),
                            },
                            // -------------------------------------------------------------
                            // LANE 5: Staggered Delay Cascade (Initial delay before launch + reverse)
                            // -------------------------------------------------------------
                            ChoreographyRacerProps {
                                title: "5. Staggered Delay",
                                feature: ".delay(0.50s) + .ping_pong()",
                                y: 400.0,
                                x: c.prop(track_start)
                                    .delay(0.50)
                                    .spring(track_end, SpringParams::from_damping_ratio(220.0, 0.40))
                                    .fit_to_duration(0.60)
                                    .hold(0.20)
                                    .ping_pong()
                                    .clone(),
                                color: c.prop(hsla(0.75, 0.88, 0.62, 1.0)),
                            },
                        ],
                        move |el, racers| {
                            let racers: Vec<ChoreographyRacerProps> = racers.to_vec();
                            el.child(
                                canvas(
                                    |_, _, _| {},
                                    move |bounds, _, window, _| {
                                        let total_dist = track_end - track_start;

                                        // -------------------------------------------------
                                        // 1. VERTICAL TIMELINE GRID (Start, 25%, 50%, 75%, Finish)
                                        // -------------------------------------------------
                                        for progress in [0.0, 0.25, 0.50, 0.75, 1.0] {
                                            let mark_x = bounds.origin.x + px(track_start + total_dist * progress + marble_size / 2.0);
                                            let is_boundary = progress == 0.0 || progress == 1.0;

                                            window.paint_quad(
                                                fill(
                                                    Bounds {
                                                        origin: point(mark_x - px(0.5), bounds.origin.y + px(30.0)),
                                                        size: size(px(if is_boundary { 2.0 } else { 1.0 }), px(430.0)),
                                                    },
                                                    hsla(0.65, 0.20, 0.35, if is_boundary { 0.45 } else { 0.15 }),
                                                )
                                            );
                                        }

                                        // -------------------------------------------------
                                        // 2. RACER LANES & MARBLES
                                        // -------------------------------------------------
                                        let white_highlight = hsla(0.0, 0.0, 1.0, 0.90);

                                        for racer in &racers {
                                            let cur_x = racer.x.get();
                                            let active_color = racer.color.get();
                                            let lane_center_y = bounds.origin.y + px(racer.y + marble_size / 2.0);

                                            // Guideline rail
                                            window.paint_quad(
                                                fill(
                                                    Bounds {
                                                        origin: point(bounds.origin.x + px(track_start), lane_center_y - px(1.0)),
                                                        size: size(px(total_dist + marble_size), px(2.0)),
                                                    },
                                                    hsla(0.65, 0.15, 0.20, 0.6),
                                                )
                                            );

                                            // Trajectory progress bar
                                            let filled_width = (cur_x - track_start).max(0.0);
                                            if filled_width > 1.0 {
                                                window.paint_quad(
                                                    fill(
                                                        Bounds {
                                                            origin: point(bounds.origin.x + px(track_start + marble_size / 2.0), lane_center_y - px(1.0)),
                                                            size: size(px(filled_width), px(2.0)),
                                                        },
                                                        hsla(active_color.h, 0.6, 0.45, 0.5),
                                                    )
                                                );
                                            }

                                            // Marble origin
                                            let marble_origin = point(
                                                bounds.origin.x + px(cur_x),
                                                bounds.origin.y + px(racer.y),
                                            );

                                            // Floor shadow
                                            let shadow_center_x = marble_origin.x + px(marble_size / 2.0);
                                            let shadow_center_y = marble_origin.y + px(marble_size + 3.0);
                                            let mut shadow_builder = PathBuilder::fill();
                                            for i in 0..24 {
                                                let theta = (i as f32 / 24.0) * 2.0 * PI;
                                                let pt = point(
                                                    shadow_center_x + px(marble_size * 0.40 * theta.cos()),
                                                    shadow_center_y + px(marble_size * 0.12 * theta.sin()),
                                                );
                                                if i == 0 {
                                                    shadow_builder.move_to(pt);
                                                } else {
                                                    shadow_builder.line_to(pt);
                                                }
                                            }
                                            if let Ok(path) = shadow_builder.build() {
                                                window.paint_path(path, hsla(0.65, 0.40, 0.04, 0.40));
                                            }

                                            // Ambient glow halo
                                            window.paint_quad(
                                                fill(
                                                    Bounds {
                                                        origin: marble_origin - point(px(3.0), px(3.0)),
                                                        size: size(px(marble_size + 6.0), px(marble_size + 6.0)),
                                                    },
                                                    hsla(active_color.h, active_color.s, active_color.l, 0.25),
                                                )
                                                .corner_radii(Corners::all(px((marble_size + 6.0) / 2.0))),
                                            );

                                            // Primary rigid sphere
                                            window.paint_quad(
                                                fill(
                                                    Bounds {
                                                        origin: marble_origin,
                                                        size: size(px(marble_size), px(marble_size)),
                                                    },
                                                    active_color,
                                                )
                                                .corner_radii(Corners::all(px(marble_size / 2.0))),
                                            );

                                            // Lower rim translucency
                                            let rim_cx = marble_origin.x + px(marble_size * 0.50);
                                            let rim_cy = marble_origin.y + px(marble_size * 0.82);
                                            let mut rim_builder = PathBuilder::fill();
                                            for i in 0..24 {
                                                let theta = (i as f32 / 24.0) * 2.0 * PI;
                                                let pt = point(
                                                    rim_cx + px(marble_size * 0.24 * theta.cos()),
                                                    rim_cy + px(marble_size * 0.07 * theta.sin()),
                                                );
                                                if i == 0 {
                                                    rim_builder.move_to(pt);
                                                } else {
                                                    rim_builder.line_to(pt);
                                                }
                                            }
                                            if let Ok(path) = rim_builder.build() {
                                                window.paint_path(path, hsla(active_color.h, 0.5, 0.90, 0.35));
                                            }

                                            // -45° Specular highlight
                                            let spec_cx = marble_origin.x + px(marble_size * 0.28);
                                            let spec_cy = marble_origin.y + px(marble_size * 0.28);
                                            let spec_rx = marble_size * 0.16;
                                            let spec_ry = marble_size * 0.09;
                                            let angle = -PI / 4.0;
                                            let cos_a = angle.cos();
                                            let sin_a = angle.sin();

                                            let mut spec_builder = PathBuilder::fill();
                                            for i in 0..24 {
                                                let theta = (i as f32 / 24.0) * 2.0 * PI;
                                                let cos_t = theta.cos();
                                                let sin_t = theta.sin();

                                                let px_val = spec_cx + px(spec_rx * cos_t * cos_a - spec_ry * sin_t * sin_a);
                                                let py_val = spec_cy + px(spec_rx * cos_t * sin_a + spec_ry * sin_t * cos_a);
                                                let pt = point(px_val, py_val);

                                                if i == 0 {
                                                    spec_builder.move_to(pt);
                                                } else {
                                                    spec_builder.line_to(pt);
                                                }
                                            }
                                            if let Ok(path) = spec_builder.build() {
                                                window.paint_path(path, white_highlight);
                                            }
                                        }
                                    },
                                )
                                .size_full(),
                            )
                        },
                    )
                    // -------------------------------------------------------------
                    // 3. OVERLAID DESCRIPTIVE LABELS
                    // -------------------------------------------------------------
                    .child(
                        div()
                            .absolute()
                            .top(px(40.0))
                            .left(px(24.0))
                            .flex()
                            .flex_col()
                            .child(div().text_sm().font_weight(FontWeight::SEMIBOLD).text_color(hsla(0.52, 0.90, 0.70, 1.0)).child("1. Natural Spring"))
                            .child(div().text_xs().text_color(hsla(0.65, 0.15, 0.60, 1.0)).child("Unconstrained (~1.85s)"))
                    )
                    .child(
                        div()
                            .absolute()
                            .top(px(130.0))
                            .left(px(24.0))
                            .flex()
                            .flex_col()
                            .child(div().text_sm().font_weight(FontWeight::SEMIBOLD).text_color(hsla(0.10, 0.95, 0.70, 1.0)).child("2. Fitted Duration"))
                            .child(div().text_xs().text_color(hsla(0.65, 0.15, 0.60, 1.0)).child(".fit_to_duration(0.75s)"))
                    )
                    .child(
                        div()
                            .absolute()
                            .top(px(220.0))
                            .left(px(24.0))
                            .flex()
                            .flex_col()
                            .child(div().text_sm().font_weight(FontWeight::SEMIBOLD).text_color(hsla(0.98, 0.90, 0.70, 1.0)).child("3. Clamped Duration"))
                            .child(div().text_xs().text_color(hsla(0.65, 0.15, 0.60, 1.0)).child(".clamp_at_duration(0.40s)"))
                    )
                    .child(
                        div()
                            .absolute()
                            .top(px(310.0))
                            .left(px(24.0))
                            .flex()
                            .flex_col()
                            .child(div().text_sm().font_weight(FontWeight::SEMIBOLD).text_color(hsla(0.38, 0.90, 0.70, 1.0)).child("4. Ping-Pong Cycle"))
                            .child(div().text_xs().text_color(hsla(0.65, 0.15, 0.60, 1.0)).child(".ping_pong() (auto reverse)"))
                    )
                    .child(
                        div()
                            .absolute()
                            .top(px(400.0))
                            .left(px(24.0))
                            .flex()
                            .flex_col()
                            .child(div().text_sm().font_weight(FontWeight::SEMIBOLD).text_color(hsla(0.75, 0.88, 0.75, 1.0)).child("5. Staggered Delay"))
                            .child(div().text_xs().text_color(hsla(0.65, 0.15, 0.60, 1.0)).child(".delay(0.50s) + .ping_pong()"))
                    ),
            )
    }
}

fn main() {
    shared::run("Temporal Choreography & Constraints", |_, _| ChoreographyShowcaseView);
}
