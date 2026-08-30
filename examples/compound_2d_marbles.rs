mod shared;

use gpui::*;
use gpui_animotion::*;
use std::f32::consts::PI;

#[derive(Clone)]
struct DynamicOrbProps {
    x: Prop<f32>,
    y: Prop<f32>,
    color: Prop<Hsla>,
}

struct CompoundMotionView;

impl Render for CompoundMotionView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let floor_y_offset = 290.0;
        let orb_size = 54.0;

        div()
            .size_full()
            .bg(hsla(0.0, 0.0, 0.95, 1.0))
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .relative()
                    .w(px(640.0))
                    .h(px(420.0))
                    .bg(hsla(0.0, 0.0, 1.0, 1.0))
                    .border_1()
                    .border_color(hsla(0.0, 0.0, 0.85, 1.0))
                    .rounded_xl()
                    .shadow_lg()
                    .animotion_clip(
                        ElementId::Name("compound_2d_marbles".into()),
                        |c| vec![
                            // Orb 1: Wide horizontal spring sway with bouncy ballistic gravity drop
                            DynamicOrbProps {
                                x: c.prop(80.0)
                                    .spring(460.0, SpringParams::from_damping_ratio(120.0, 0.35))
                                    .spring(80.0, SpringParams::from_damping_ratio(120.0, 0.50))
                                    .clone(),
                                y: c.prop(30.0)
                                    .gravity(
                                        GravityParams {
                                            gravity: 2100.0,
                                            restitution: 0.78,
                                            floor_y: floor_y_offset,
                                            ..Default::default()
                                        },
                                        5,
                                    )
                                    .tween(30.0, 0.9)
                                    .clone(),
                                color: c.prop(hsla(0.60, 0.85, 0.55, 1.0))
                                    .tween(hsla(0.85, 0.85, 0.60, 1.0), 1.4)
                                    .tween(hsla(0.60, 0.85, 0.55, 1.0), 1.4)
                                    .clone(),
                            },
                            // Orb 2: Counter-phase horizontal swing with heavy damped gravity drop
                            DynamicOrbProps {
                                x: c.prop(460.0)
                                    .spring(100.0, SpringParams::from_damping_ratio(140.0, 0.40))
                                    .spring(460.0, SpringParams::from_damping_ratio(140.0, 0.60))
                                    .clone(),
                                y: c.prop(50.0)
                                    .gravity(
                                        GravityParams {
                                            gravity: 2600.0,
                                            restitution: 0.62,
                                            floor_y: floor_y_offset,
                                            ..Default::default()
                                        },
                                        4,
                                    )
                                    .tween(50.0, 0.9)
                                    .clone(),
                                color: c.prop(hsla(0.12, 0.90, 0.50, 1.0))
                                    .tween(hsla(0.32, 0.85, 0.45, 1.0), 1.2)
                                    .tween(hsla(0.12, 0.90, 0.50, 1.0), 1.2)
                                    .clone(),
                            },
                        ],
                        move |el, orbs| {
                            let orbs: Vec<DynamicOrbProps> = orbs.to_vec();
                            el.child(
                                canvas(
                                    |_, _, _| {},
                                    move |bounds, _, window, _| {
                                        let white_highlight = hsla(0.0, 0.0, 1.0, 0.65);

                                        for orb in &orbs {
                                            let current_x = orb.x.get();
                                            let current_y = orb.y.get();

                                            let ground_x = bounds.origin.x + px(current_x + orb_size / 2.0);
                                            let ground_y = bounds.origin.y + px(floor_y_offset + orb_size);

                                            let max_height = 30.0;
                                            let height_range = floor_y_offset - max_height;
                                            let current_dist = floor_y_offset - current_y;
                                            let proximity = (1.0 - (current_dist / height_range)).clamp(0.0, 1.0);

                                            // Dynamic Dynamic Floor Shadow
                                            let shadow_rx = (orb_size * 0.18) + (orb_size * 0.36 * proximity);
                                            let shadow_ry = (orb_size * 0.05) + (orb_size * 0.08 * proximity);
                                            let shadow_alpha = 0.04 + (0.32 * proximity);

                                            let mut shadow_builder = PathBuilder::fill();
                                            for i in 0..24 {
                                                let theta = (i as f32 / 24.0) * 2.0 * PI;
                                                let pt = point(
                                                    ground_x + px(shadow_rx * theta.cos()),
                                                    ground_y + px(shadow_ry * theta.sin()),
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

                                            // Rigid Sphere Body
                                            let orb_origin = point(
                                                bounds.origin.x + px(current_x),
                                                bounds.origin.y + px(current_y),
                                            );

                                            window.paint_quad(
                                                fill(
                                                    Bounds {
                                                        origin: orb_origin,
                                                        size: size(px(orb_size), px(orb_size)),
                                                    },
                                                    orb.color.get(),
                                                )
                                                .corner_radii(Corners::all(px(orb_size / 2.0))),
                                            );

                                            // Specular Highlight (-45° Reflection)
                                            let cx = orb_origin.x + px(orb_size * 0.28);
                                            let cy = orb_origin.y + px(orb_size * 0.28);
                                            let rx = orb_size * 0.16;
                                            let ry = orb_size * 0.09;
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
    shared::run("Compound 2D Ballistic & Spring Motion", |_, _| CompoundMotionView);
}
