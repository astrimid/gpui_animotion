mod shared;

use gpui::*;
use gpui_animotion::*;
use std::f32::consts::PI;

#[derive(Clone)]
struct PuckProps {
    y: f32,
    x: Prop<f32>,
    color: Prop<Hsla>,
}

struct KineticFlickView;

impl Render for KineticFlickView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let track_start_x = 60.0;
        let track_end_x = 480.0;
        let puck_size = 56.0;

        div()
            .size_full()
            .bg(hsla(0.0, 0.0, 0.95, 1.0))
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .relative()
                    .w(px(600.0))
                    .h(px(400.0))
                    .bg(hsla(0.0, 0.0, 1.0, 1.0))
                    .border_1()
                    .border_color(hsla(0.0, 0.0, 0.85, 1.0))
                    .rounded_xl()
                    .shadow_lg()
                    .animotion_clip(
                        ElementId::Name("physics_kinetic_flick_pucks".into()),
                        |c| vec![
                            // 1. High Velocity / Low Friction Glide (Long drift into elastic spring return)
                            PuckProps {
                                y: 80.0,
                                x: c.prop(track_start_x)
                                    .flick(FlickParams {
                                        initial_velocity: 2800.0,
                                        friction: 2.4,
                                        threshold: 0.5,
                                        duration: None,
                                    })
                                    .spring(
                                        track_start_x,
                                        SpringParams::from_damping_ratio(220.0, 0.45),
                                    )
                                    .clone(),
                                color: c.prop(hsla(0.58, 0.85, 0.55, 1.0)),
                            },
                            // 2. Medium Velocity / Standard UI Friction (Snappy drag-release feel)
                            PuckProps {
                                y: 172.0,
                                x: c.prop(track_start_x)
                                    .flick(FlickParams {
                                        initial_velocity: 2000.0,
                                        friction: 4.2,
                                        threshold: 0.5,
                                        duration: None,
                                    })
                                    .spring(
                                        track_start_x,
                                        SpringParams::from_damping_ratio(190.0, 0.65),
                                    )
                                    .clone(),
                                color: c.prop(hsla(0.38, 0.80, 0.45, 1.0)),
                            },
                            // 3. Low Velocity / Heavy Friction Brake (Gentle glide with tight snap)
                            PuckProps {
                                y: 264.0,
                                x: c.prop(track_start_x)
                                    .flick(FlickParams {
                                        initial_velocity: 1400.0,
                                        friction: 6.8,
                                        threshold: 0.5,
                                        duration: None,
                                    })
                                    .spring(
                                        track_start_x,
                                        SpringParams::from_damping_ratio(240.0, 0.80),
                                    )
                                    .clone(),
                                color: c.prop(hsla(0.08, 0.85, 0.55, 1.0)),
                            },
                        ],
                        move |el, pucks| {
                            let pucks: Vec<PuckProps> = pucks.to_vec();
                            el.child(
                                canvas(
                                    |_, _, _| {},
                                    move |bounds, _, window, _| {
                                        let white_highlight = hsla(0.0, 0.0, 1.0, 0.65);
                                        let total_travel = track_end_x - track_start_x;

                                        for puck in &pucks {
                                            let current_x = puck.x.get();

                                            // Runway guideline
                                            let lane_y = bounds.origin.y + px(puck.y + puck_size / 2.0);
                                            window.paint_quad(
                                                fill(
                                                    Bounds {
                                                        origin: point(bounds.origin.x + px(track_start_x), lane_y - px(1.0)),
                                                        size: size(px(total_travel + puck_size), px(2.0)),
                                                    },
                                                    hsla(0.0, 0.0, 0.90, 1.0),
                                                )
                                            );

                                            // Ground Shadow
                                            let normalized_pos = ((current_x - track_start_x) / total_travel).clamp(0.0, 1.0);
                                            let shadow_center_x = bounds.origin.x + px(current_x + puck_size / 2.0);
                                            let shadow_center_y = bounds.origin.y + px(puck.y + puck_size + 4.0);

                                            let shadow_rx = puck_size * 0.42;
                                            let shadow_ry = puck_size * 0.10;
                                            let shadow_alpha = 0.08 + (0.18 * (1.0 - normalized_pos * 0.5));

                                            let mut shadow_builder = PathBuilder::fill();
                                            for i in 0..24 {
                                                let theta = (i as f32 / 24.0) * 2.0 * PI;
                                                let pt = point(
                                                    shadow_center_x + px(shadow_rx * theta.cos()),
                                                    shadow_center_y + px(shadow_ry * theta.sin()),
                                                );
                                                if i == 0 {
                                                    shadow_builder.move_to(pt);
                                                } else {
                                                    shadow_builder.line_to(pt);
                                                }
                                            }
                                            if let Ok(path) = shadow_builder.build() {
                                                window.paint_path(path, hsla(0.0, 0.0, 0.0, shadow_alpha));
                                            }

                                            // Rigid Puck Body
                                            let puck_origin = point(
                                                bounds.origin.x + px(current_x),
                                                bounds.origin.y + px(puck.y),
                                            );

                                            window.paint_quad(
                                                fill(
                                                    Bounds {
                                                        origin: puck_origin,
                                                        size: size(px(puck_size), px(puck_size)),
                                                    },
                                                    puck.color.get(),
                                                )
                                                .corner_radii(Corners::all(px(puck_size / 2.0))),
                                            );

                                            // Specular Highlight (-45° Reflection)
                                            let cx = puck_origin.x + px(puck_size * 0.28);
                                            let cy = puck_origin.y + px(puck_size * 0.28);
                                            let rx = puck_size * 0.16;
                                            let ry = puck_size * 0.09;
                                            let angle = -PI / 4.0;

                                            let cos_a = angle.cos();
                                            let sin_a = angle.sin();

                                            let mut highlight_builder = PathBuilder::fill();
                                            for i in 0..24 {
                                                let theta = (i as f32 / 24.0) * 2.0 * PI;
                                                let cos_t = theta.cos();
                                                let sin_t = theta.sin();

                                                let px_val = cx + px(rx * cos_t * cos_a - ry * sin_t * sin_a);
                                                let py_val = cy + px(rx * cos_t * sin_a + ry * sin_t * cos_a);
                                                let pt = point(px_val, py_val);

                                                if i == 0 {
                                                    highlight_builder.move_to(pt);
                                                } else {
                                                    highlight_builder.line_to(pt);
                                                }
                                            }

                                            if let Ok(path) = highlight_builder.build() {
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
    shared::run("Kinetic Friction Flick & Spring Return", |_, _| KineticFlickView);
}
