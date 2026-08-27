mod shared;

use gpui::*;
use gpui_animotion::*;
use std::f32::consts::PI;

#[derive(Clone)]
struct BallProps {
    x: f32,
    y: Prop<f32>,
    color: Prop<Hsla>,
}

struct BouncingBallView;

impl Render for BouncingBallView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let floor_y_offset = 300.0;
        let ball_size = 60.0;

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
                        ElementId::Name("physics_gravity_marbles".into()),
                        |c| vec![
                            // Glass Marble: High restitution (bouncy)
                            BallProps {
                                x: 100.0,
                                y: c.prop(40.0)
                                    .gravity(
                                        GravityParams {
                                            gravity: 2400.0,
                                            restitution: 0.80,
                                            floor_y: floor_y_offset,
                                            ..Default::default()
                                        },
                                        6,
                                    )
                                    .tween(40.0, 1.0)
                                    .clone(),
                                color: c.prop(hsla(0.75, 0.8, 0.50, 1.0)),
                            },
                            // Rubber Ball: Medium gravity, high restitution
                            BallProps {
                                x: 260.0,
                                y: c.prop(80.0)
                                    .gravity(
                                        GravityParams {
                                            gravity: 1800.0,
                                            restitution: 0.88,
                                            floor_y: floor_y_offset,
                                            ..Default::default()
                                        },
                                        7,
                                    )
                                    .tween(80.0, 1.2)
                                    .clone(),
                                color: c.prop(hsla(0.25, 0.8, 0.45, 1.0)),
                            },
                            // Heavy Ball: High gravity, low restitution (damped bounce)
                            BallProps {
                                x: 420.0,
                                y: c.prop(20.0)
                                    .gravity(
                                        GravityParams {
                                            gravity: 3200.0,
                                            restitution: 0.55,
                                            floor_y: floor_y_offset,
                                            ..Default::default()
                                        },
                                        4,
                                    )
                                    .tween(20.0, 0.8)
                                    .clone(),
                                color: c.prop(hsla(0.55, 0.8, 0.50, 1.0)),
                            },
                        ],
                        move |el, balls| {
                            let balls: Vec<BallProps> = balls.to_vec();
                            el.child(
                                canvas(
                                    |_, _, _| {},
                                    move |bounds, _, window, _| {
                                        let white_highlight = hsla(0.0, 0.0, 1.0, 0.65);

                                        for ball in &balls {
                                            let current_y = ball.y.get();

                                            let ground_x = bounds.origin.x + px(ball.x + ball_size / 2.0);
                                            let ground_y = bounds.origin.y + px(floor_y_offset + ball_size);

                                            let max_height = 20.0;
                                            let height_range = floor_y_offset - max_height;
                                            let current_dist = floor_y_offset - current_y;
                                            let proximity = (1.0 - (current_dist / height_range)).clamp(0.0, 1.0);

                                            // Floor Shadow
                                            let shadow_rx = (ball_size * 0.15) + (ball_size * 0.35 * proximity);
                                            let shadow_ry = (ball_size * 0.04) + (ball_size * 0.07 * proximity);
                                            let shadow_alpha = 0.04 + (0.35 * proximity);

                                            let mut shadow_builder = PathBuilder::fill();
                                            for i in 0..24 {
                                                let theta = (i as f32 / 24.0) * 2.0 * PI;
                                                let pt = point(
                                                    ground_x + px(shadow_rx * theta.cos()),
                                                    ground_y + px(shadow_ry * theta.sin()),
                                                );
                                                if i == 0 { shadow_builder.move_to(pt); } 
                                                else { shadow_builder.line_to(pt); }
                                            }
                                            if let Ok(path) = shadow_builder.build() {
                                                window.paint_path(path, hsla(0.0, 0.0, 0.0, shadow_alpha));
                                            }

                                            // Rigid Sphere
                                            let ball_origin = point(
                                                bounds.origin.x + px(ball.x),
                                                bounds.origin.y + px(current_y),
                                            );

                                            window.paint_quad(
                                                fill(
                                                    Bounds {
                                                        origin: ball_origin,
                                                        size: size(px(ball_size), px(ball_size)),
                                                    },
                                                    ball.color.get(),
                                                )
                                                .corner_radii(Corners::all(px(ball_size / 2.0))),
                                            );

                                            // Specular Highlight (-45° Reflection)
                                            let cx = ball_origin.x + px(ball_size * 0.28);
                                            let cy = ball_origin.y + px(ball_size * 0.28);
                                            let rx = ball_size * 0.16;
                                            let ry = ball_size * 0.09;
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

                                                if i == 0 { highlight_builder.move_to(pt); } 
                                                else { highlight_builder.line_to(pt); }
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
    shared::run("Gravitational Physics Marbles", |_, _| BouncingBallView);
}
