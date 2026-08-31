mod shared;

use gpui::*;
use gpui_animotion::*;
use std::f32::consts::PI;

#[derive(Clone)]
struct MarbleProps {
    x: Prop<f32>,
    y: Prop<f32>,
    color: Prop<Hsla>,
    label: &'static str,
}

struct MarblePlaygroundView;

impl Render for MarblePlaygroundView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let floor_y = 290.0;
        let marble_size = 52.0;

        div()
            .size_full()
            .bg(hsla(0.62, 0.30, 0.08, 1.0)) // Deep arcade midnight background
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .child(
                // Arena Glass Container
                div()
                    .relative()
                    .w(px(720.0))
                    .h(px(460.0))
                    .bg(hsla(0.62, 0.25, 0.12, 0.95))
                    .border_1()
                    .border_color(hsla(0.62, 0.30, 0.25, 0.8))
                    .rounded_2xl()
                    .shadow_xl()
                    .animotion_clip(
                        ElementId::Name("kinetic_marble_playground".into()),
                        |c| vec![
                            // -------------------------------------------------------------
                            // 1. SOLAR FLARE (Orange): Parabolic Gravity Drop & Spring Catapult
                            // -------------------------------------------------------------
                            MarbleProps {
                                label: "Ballistic Gravity",
                                x: c.prop(100.0)
                                    .tween(130.0, 1.4)
                                    .ease(Ease::InOutQuad)
                                    .tween(100.0, 1.8)
                                    .ease(Ease::InOutQuad)
                                    .clone(),
                                y: c.prop(30.0)
                                    // Step A: 4-bounce kinematic ballistic drop
                                    .gravity(
                                        GravityParams {
                                            gravity: 2400.0,
                                            restitution: 0.78,
                                            floor_y,
                                            ..Default::default()
                                        },
                                        4,
                                    )
                                    // Step B: Elastic spring recoil catapult
                                    .spring(30.0, SpringParams::from_damping_ratio(220.0, 0.55))
                                    .clone(),
                                color: c.prop(hsla(0.08, 0.95, 0.58, 1.0))
                                    .tween(hsla(0.14, 0.95, 0.62, 1.0), 1.6)
                                    .tween(hsla(0.08, 0.95, 0.58, 1.0), 1.6)
                                    .clone(),
                            },
                            // -------------------------------------------------------------
                            // 2. CYBER COMET (Cyan): High-Velocity Flick & Bumper Spring Bounce
                            // -------------------------------------------------------------
                            MarbleProps {
                                label: "Kinetic Flick + Bumper",
                                x: c.prop(120.0)
                                    // Step A: High-velocity flick truncated at the bumper face (x = 568.0 at t = 0.35s)
                                    .flick(FlickParams {
                                        initial_velocity: 2100.0,
                                        friction: 2.8,
                                        threshold: 0.5,
                                        duration: Some(1.7),
                                    })
                                    // Step B: Hits the bumper and springs back to the launch position
                                    .spring(120.0, SpringParams::from_damping_ratio(180.0, 0.40))
                                    .clone(),
                                y: c.prop(160.0),
                                color: c.prop(hsla(0.52, 0.95, 0.56, 1.0))
                            },
                            // -------------------------------------------------------------
                            // 3. EMERALD PULSAR (Green): Cubic Bezier Anticipation & Damped Oscillation
                            // -------------------------------------------------------------
                            MarbleProps {
                                label: "Cubic Bezier Easing",
                                x: c.prop(300.0)
                                    // Step A: Custom anticipation & overshoot curve (CSS-style)
                                    .tween(540.0, 1.4)
                                    .ease(Ease::Custom(0.68, -0.55, 0.27, 1.55))
                                    // Step B: Smooth cubic return
                                    .tween_eased(300.0, 1.6, Ease::InOutCubic)
                                    .clone(),
                                y: c.prop(90.0)
                                    // Harmonic underdamped vertical oscillation
                                    .spring(floor_y - 30.0, SpringParams::from_damping_ratio(190.0, 0.30))
                                    .spring(90.0, SpringParams::from_damping_ratio(160.0, 0.65))
                                    .clone(),
                                color: c.prop(hsla(0.38, 0.90, 0.50, 1.0)),
                            },
                            // -------------------------------------------------------------
                            // 4. NEON PHANTOM (Purple): Exponential InOut Wave with Zero Friction
                            // -------------------------------------------------------------
                            MarbleProps {
                                label: "Exponential Transfer",
                                x: c.prop(480.0)
                                    .tween(240.0, 1.5)
                                    .ease(Ease::InOutExpo)
                                    .tween(480.0, 1.5)
                                    .ease(Ease::InOutExpo)
                                    .clone(),
                                y: c.prop(220.0)
                                    .spring(50.0, SpringParams::from_damping_ratio(240.0, 0.28))
                                    .spring(220.0, SpringParams::from_damping_ratio(200.0, 0.75))
                                    .clone(),
                                color: c.prop(hsla(0.78, 0.90, 0.60, 1.0))
                                    .tween(hsla(0.92, 0.90, 0.60, 1.5), 1.5)
                                    .ease(Ease::InOutExpo)
                                    .tween(hsla(0.78, 0.90, 0.60, 1.5), 1.5)
                                    .ease(Ease::InOutExpo)
                                    .clone(),
                            },
                        ],
                        move |el, marbles| {
                            let marbles: Vec<MarbleProps> = marbles.to_vec();
                            el.child(
                                canvas(
                                    |_, _, _| {},
                                    move |bounds, _, window, _| {
                                        // -------------------------------------------------
                                        // 1. ARENA SCENERY: Rails, Bumpers, and Baseline Floor
                                        // -------------------------------------------------
                                        let floor_screen_y = bounds.origin.y + px(floor_y + marble_size);

                                        // Neon Grid Floor Plane
                                        window.paint_quad(
                                            fill(
                                                Bounds {
                                                    origin: point(bounds.origin.x + px(30.0), floor_screen_y),
                                                    size: size(px(660.0), px(3.0)),
                                                },
                                                hsla(0.62, 0.40, 0.35, 0.8),
                                            )
                                        );

                                        // Kinetic Bumper Obstacle at right boundary
                                        let bumper_x = bounds.origin.x + px(620.0);
                                        let bumper_top_y = bounds.origin.y + px(120.0);
                                        window.paint_quad(
                                            fill(
                                                Bounds {
                                                    origin: point(bumper_x, bumper_top_y),
                                                    size: size(px(14.0), px(120.0)),
                                                },
                                                hsla(0.52, 0.90, 0.65, 0.9),
                                            )
                                            .corner_radii(Corners::all(px(7.0))),
                                        );

                                        // Horizontal Guide Rails (Subtle backdrop paths)
                                        for lane_y in [160.0, 220.0] {
                                            window.paint_quad(
                                                fill(
                                                    Bounds {
                                                        origin: point(bounds.origin.x + px(50.0), bounds.origin.y + px(lane_y + marble_size / 2.0)),
                                                        size: size(px(620.0), px(1.0)),
                                                    },
                                                    hsla(0.62, 0.20, 0.22, 0.4),
                                                )
                                            );
                                        }

                                        // -------------------------------------------------
                                        // 2. MARBLE BODIES, DYNAMIC SHADOWS & SPECULAR HIGHLIGHTS
                                        // -------------------------------------------------
                                        let white_highlight = hsla(0.0, 0.0, 1.0, 0.85);

                                        for marble in &marbles {
                                            let cur_x = marble.x.get();
                                            let cur_y = marble.y.get();
                                            let active_color = marble.color.get();

                                            let shadow_center_x = bounds.origin.x + px(cur_x + marble_size / 2.0);
                                            let shadow_center_y = bounds.origin.y + px(floor_y + marble_size + 4.0);

                                            // Altitude Proximity (0.0 = high in air, 1.0 = touching floor)
                                            let altitude = (floor_y - cur_y).max(0.0);
                                            let max_flight = floor_y - 30.0;
                                            let proximity = (1.0 - (altitude / max_flight)).clamp(0.0, 1.0);

                                            // A. Floor Shadow (expands & deepens on landing)
                                            let shadow_rx = (marble_size * 0.16) + (marble_size * 0.38 * proximity);
                                            let shadow_ry = (marble_size * 0.04) + (marble_size * 0.09 * proximity);
                                            let shadow_alpha = 0.05 + (0.45 * proximity);

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
                                                window.paint_path(path, hsla(0.62, 0.40, 0.04, shadow_alpha));
                                            }

                                            // B. Ambient Halo Glow
                                            let marble_origin = point(
                                                bounds.origin.x + px(cur_x),
                                                bounds.origin.y + px(cur_y),
                                            );

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

                                            // C. Primary Rigid Marble Sphere
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

                                            // D. Bottom Rim Light (Glass Translucency)
                                            let rim_cx = marble_origin.x + px(marble_size * 0.50);
                                            let rim_cy = marble_origin.y + px(marble_size * 0.82);
                                            let rim_rx = marble_size * 0.25;
                                            let rim_ry = marble_size * 0.08;

                                            let mut rim_builder = PathBuilder::fill();
                                            for i in 0..24 {
                                                let theta = (i as f32 / 24.0) * 2.0 * PI;
                                                let pt = point(
                                                    rim_cx + px(rim_rx * theta.cos()),
                                                    rim_cy + px(rim_ry * theta.sin()),
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

                                            // E. Primary -45° Specular Highlight
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
    shared::run("Kinetic Marble Odyssey: Multi-Primitive Sandbox", |_, _| MarblePlaygroundView);
}
