mod shared;

use gpui::*;
use gpui_animotion::*;
use std::f32::consts::PI;

#[derive(Clone)]
struct EasingRacerProps {
    x: Prop<f32>,
    color: Prop<Hsla>,
    title: &'static str,
    formula: &'static str,
    y: f32,
}

struct EasingShowcaseView;

impl Render for EasingShowcaseView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let track_start = 180.0;
        let track_end = 660.0;
        let marble_size = 36.0;

        div()
            .size_full()
            .bg(hsla(0.65, 0.28, 0.08, 1.0))
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .child(
                div()
                    .relative()
                    .w(px(740.0))
                    .h(px(520.0))
                    .bg(hsla(0.65, 0.22, 0.12, 0.96))
                    .border_1()
                    .border_color(hsla(0.65, 0.25, 0.24, 0.8))
                    .rounded_2xl()
                    .shadow_xl()
                    .animotion_clip(
                        ElementId::Name("easing_bezier_racetrack".into()),
                        |c| vec![
                            // -------------------------------------------------------------
                            // 1. LINEAR: Constant velocity benchmark
                            // -------------------------------------------------------------
                            EasingRacerProps {
                                title: "1. Linear",
                                formula: "f(t) = t",
                                y: 50.0,
                                x: c.prop(track_start)
                                    .tween(track_end, 2.0)
                                    .ease(Ease::Linear)
                                    .tween(track_start, 1.0)
                                    .ease(Ease::Linear)
                                    .clone(),
                                color: c.prop(hsla(0.60, 0.15, 0.70, 1.0)),
                            },
                            // -------------------------------------------------------------
                            // 2. EASE-IN-OUT CUBIC: Standard smooth UI acceleration/deceleration
                            // -------------------------------------------------------------
                            EasingRacerProps {
                                title: "2. InOut Cubic",
                                formula: "cubic-bezier(0.65, 0, 0.35, 1)",
                                y: 125.0,
                                x: c.prop(track_start)
                                    .tween(track_end, 2.0)
                                    .ease(Ease::InOutCubic)
                                    .tween(track_start, 1.0)
                                    .ease(Ease::InOutQuad)
                                    .clone(),
                                color: c.prop(hsla(0.55, 0.90, 0.55, 1.0)),
                            },
                            // -------------------------------------------------------------
                            // 3. IN-OUT BACK: Dual-sided anticipation & overshoot
                            // -------------------------------------------------------------
                            EasingRacerProps {
                                title: "3. InOut Back",
                                formula: "Pullback -> Overshoot",
                                y: 200.0,
                                x: c.prop(track_start)
                                    .tween(track_end, 2.0)
                                    .ease(Ease::InOutBack)
                                    .tween(track_start, 1.0)
                                    .ease(Ease::OutQuad)
                                    .clone(),
                                color: c.prop(hsla(0.08, 0.95, 0.58, 1.0)),
                            },
                            // -------------------------------------------------------------
                            // 4. MAGNETIC RAILGUN (No Overshoot): Hypersonic blast -> Hard magnetic brake lock
                            // -------------------------------------------------------------
                            EasingRacerProps {
                                title: "4. Railgun (Clamped)",
                                formula: "cubic-bezier(0.05, 1.00, 0.10, 1.00)",
                                y: 275.0,
                                x: c.prop(track_start)
                                    // Forward: Instant velocity punch to 98% track distance in ~100ms, clamping smoothly at 100%
                                    .tween(track_end, 2.0)
                                    .ease(Ease::Custom(0.05, 1.00, 0.10, 1.00))
                                    // Return: Snappy reset
                                    .tween(track_start, 1.0)
                                    .ease(Ease::Custom(0.20, 0.00, 0.10, 1.00))
                                    .clone(),
                                color: c.prop(hsla(0.80, 0.92, 0.62, 1.0)),
                            },
                            // -------------------------------------------------------------
                            // 5. IN-OUT EXPO: Extended holding time with hyper-velocity transfer
                            // -------------------------------------------------------------
                            EasingRacerProps {
                                title: "5. InOut Expo",
                                formula: "f(t) = 2^(10(2t-1))",
                                y: 350.0,
                                x: c.prop(track_start)
                                    .tween(track_end, 2.0)
                                    .ease(Ease::InOutExpo)
                                    .tween(track_start, 1.0)
                                    .ease(Ease::InOutExpo)
                                    .clone(),
                                color: c.prop(hsla(0.32, 0.90, 0.52, 1.0)),
                            },
                            // -------------------------------------------------------------
                            // 6. OUT BOUNCE: Multi-rebound ballistic terminal decay
                            // -------------------------------------------------------------
                            EasingRacerProps {
                                title: "6. Out Bounce",
                                formula: "Piecewise quadratic decay",
                                y: 425.0,
                                x: c.prop(track_start)
                                    .tween(track_end, 2.0)
                                    .ease(Ease::OutBounce)
                                    .tween(track_start, 1.0)
                                    .ease(Ease::InQuad)
                                    .clone(),
                                color: c.prop(hsla(0.14, 0.95, 0.55, 1.0)),
                            },
                        ],
                        move |el, racers| {
                            let racers: Vec<EasingRacerProps> = racers.to_vec();
                            let total_dist = track_end - track_start;

                            el
                                // ---------------------------------------------------------
                                // A. TRACK MILESTONE LABELS (Top Grid: 0%, 25%, 50%, 75%, 100%)
                                // ---------------------------------------------------------
                                .children([
                                    (0.0, "START"),
                                    (0.25, "25%"),
                                    (0.50, "50%"),
                                    (0.75, "75%"),
                                    (1.0, "FINISH"),
                                ].into_iter().map(|(progress, label)| {
                                    let center_x = track_start + total_dist * progress + marble_size / 2.0;
                                    div()
                                        .absolute()
                                        .left(px(center_x - 36.0))
                                        .top(px(14.0))
                                        .w(px(72.0))
                                        .flex()
                                        .justify_center()
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(hsla(0.65, 0.20, 0.55, 0.8))
                                                .child(label)
                                        )
                                }))
                                // ---------------------------------------------------------
                                // B. LANE IDENTIFIERS & FORMULAS (Left Margin: 0..170px)
                                // ---------------------------------------------------------
                                .children(racers.iter().map(|racer| {
                                    let active_color = racer.color.get();
                                    div()
                                        .absolute()
                                        .left(px(20.0))
                                        .top(px(racer.y - 1.0))
                                        .w(px(150.0))
                                        .flex()
                                        .flex_col()
                                        .gap(px(2.0))
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap(px(6.0))
                                                .child(
                                                    // Dynamic Color Pip
                                                    div()
                                                        .size(px(7.0))
                                                        .rounded_full()
                                                        .bg(active_color)
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .text_color(hsla(0.0, 0.0, 0.95, 0.95))
                                                        .child(racer.title)
                                                )
                                        )
                                        .child(
                                            div()
                                                .pl(px(13.0))
                                                .text_xs()
                                                .text_color(hsla(0.65, 0.15, 0.60, 0.75))
                                                .child(racer.formula)
                                        )
                                }))
                                // ---------------------------------------------------------
                                // C. CANVAS SCENERY, GUIDELINES, AND MARBLES
                                // ---------------------------------------------------------
                                .child(
                                    canvas(
                                        |_, _, _| {},
                                        move |bounds, _, window, _| {
                                            // 1. Grid Markers
                                            for progress in [0.0, 0.25, 0.50, 0.75, 1.0] {
                                                let mark_x = bounds.origin.x + px(track_start + total_dist * progress + marble_size / 2.0);
                                                let is_boundary = progress == 0.0 || progress == 1.0;

                                                window.paint_quad(
                                                    fill(
                                                        Bounds {
                                                            origin: point(mark_x - px(0.5), bounds.origin.y + px(35.0)),
                                                            size: size(px(if is_boundary { 2.0 } else { 1.0 }), px(440.0)),
                                                        },
                                                        hsla(0.65, 0.20, 0.35, if is_boundary { 0.45 } else { 0.15 }),
                                                    )
                                                );
                                            }

                                            // 2. Racer Lanes and Marbles
                                            let white_highlight = hsla(0.0, 0.0, 1.0, 0.90);

                                            for racer in &racers {
                                                let cur_x = racer.x.get();
                                                let active_color = racer.color.get();
                                                let lane_center_y = bounds.origin.y + px(racer.y + marble_size / 2.0);

                                                // Lane Track Guideline
                                                window.paint_quad(
                                                    fill(
                                                        Bounds {
                                                            origin: point(bounds.origin.x + px(track_start), lane_center_y - px(1.0)),
                                                            size: size(px(total_dist + marble_size), px(2.0)),
                                                        },
                                                        hsla(0.65, 0.15, 0.20, 0.6),
                                                    )
                                                );

                                                // Travel Progress Fill
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

                                                // Marble Origin Coordinates
                                                let marble_origin = point(
                                                    bounds.origin.x + px(cur_x),
                                                    bounds.origin.y + px(racer.y),
                                                );

                                                // Cast Floor Shadow
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

                                                // Ambient Glow Halo
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

                                                // Marble Core Sphere
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

                                                // Translucent Lower-Rim Lighting
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

                                                // Specular Reflection (-45°)
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
                    ),
            )
    }
}

fn main() {
    shared::run("Easing Curves & Cubic Bezier Racetrack", |_, _| EasingShowcaseView);
}
