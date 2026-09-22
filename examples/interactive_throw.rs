mod shared;

use gpui::*;
use gpui_animotion::*;
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};

const HOME_X: f32 = 356.0;
const HOME_Y: f32 = 226.0;
const MARBLE_SIZE: f32 = 54.0;

struct InteractiveThrowView {
    marble_x: Prop<f32>,
    marble_y: Prop<f32>,
    marble_color: Prop<Hsla>,
    arena_origin: Arc<Mutex<Point<Pixels>>>,
    tracker: VelocityTracker2D,
    is_dragging: bool,
    drag_offset: Point<f32>,
    last_release_velocity: (f32, f32),
    status_label: &'static str,
}

impl InteractiveThrowView {
    pub fn new() -> Self {
        Self {
            marble_x: Prop::new(HOME_X),
            marble_y: Prop::new(HOME_Y),
            marble_color: Prop::new(hsla(0.48, 0.95, 0.55, 1.0)),
            arena_origin: Arc::new(Mutex::new(point(px(0.0), px(0.0)))),
            tracker: VelocityTracker2D::default(),
            is_dragging: false,
            drag_offset: point(0.0, 0.0),
            last_release_velocity: (0.0, 0.0),
            status_label: "RESTING AT DOCK",
        }
    }
}

impl Render for InteractiveThrowView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let arena_origin = Arc::clone(&self.arena_origin);
        let marble_x = self.marble_x.clone();
        let marble_y = self.marble_y.clone();
        let marble_color = self.marble_color.clone();

        let is_dragging = self.is_dragging;
        let last_vel = self.last_release_velocity;
        let status = self.status_label;

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
                    .w(px(760.0))
                    .h(px(500.0))
                    .bg(hsla(0.65, 0.22, 0.12, 0.96))
                    .border_1()
                    .border_color(hsla(0.65, 0.25, 0.24, 0.8))
                    .rounded_2xl()
                    .shadow_xl()
                    // -------------------------------------------------------------
                    // MOUSE GESTURE LISTENERS
                    // -------------------------------------------------------------
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, event: &MouseDownEvent, _window, cx| {
                            let origin = *this.arena_origin.lock().unwrap();
                            let local_x = f32::from(event.position.x - origin.x);
                            let local_y = f32::from(event.position.y - origin.y);

                            let cur_x = this.marble_x.get();
                            let cur_y = this.marble_y.get();
                            let center_x = cur_x + MARBLE_SIZE / 2.0;
                            let center_y = cur_y + MARBLE_SIZE / 2.0;

                            let dist = ((local_x - center_x).powi(2) + (local_y - center_y).powi(2)).sqrt();

                            if dist <= (MARBLE_SIZE / 2.0 + 12.0) {
                                this.is_dragging = true;
                                this.marble_x.stop();
                                this.marble_y.stop();
                                this.tracker.reset();
                                this.tracker.push(local_x, local_y);
                                this.drag_offset = point(local_x - cur_x, local_y - cur_y);
                                this.status_label = "DRAGGING (RECORDING VELOCITY)";
                                cx.notify();
                            }
                        }),
                    )
                    .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _window, cx| {
                        if this.is_dragging {
                            let origin = *this.arena_origin.lock().unwrap();
                            let local_x = f32::from(event.position.x - origin.x);
                            let local_y = f32::from(event.position.y - origin.y);

                            this.tracker.push(local_x, local_y);

                            let target_x = (local_x - this.drag_offset.x).clamp(24.0, 760.0 - MARBLE_SIZE - 24.0);
                            let target_y = (local_y - this.drag_offset.y).clamp(24.0, 500.0 - MARBLE_SIZE - 24.0);

                            this.marble_x.interrupt_tween(target_x, 0.001, Ease::Linear);
                            this.marble_y.interrupt_tween(target_y, 0.001, Ease::Linear);

                            cx.notify();
                        }
                    }))
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(|this, _event: &MouseUpEvent, _window, cx| {
                            if this.is_dragging {
                                this.is_dragging = false;

                                let (mut vx, mut vy) = this.tracker.velocity();
                                let max_v = 4500.0;
                                vx = vx.clamp(-max_v, max_v);
                                vy = vy.clamp(-max_v, max_v);
                                this.last_release_velocity = (vx, vy);

                                let speed = (vx * vx + vy * vy).sqrt();

                                if speed > 120.0 {
                                    this.status_label = "FLICK GLIDE -> SPRING RETURN";

                                    this.marble_x
                                        .interrupt_flick(FlickParams {
                                            initial_velocity: vx,
                                            friction: 3.2,
                                            threshold: 1.0,
                                            duration: None,
                                        })
                                        .spring(
                                            HOME_X,
                                            SpringParams::from_damping_ratio(150.0, 0.45),
                                        );

                                    this.marble_y
                                        .interrupt_flick(FlickParams {
                                            initial_velocity: vy,
                                            friction: 3.2,
                                            threshold: 1.0,
                                            duration: None,
                                        })
                                        .spring(
                                            HOME_Y,
                                            SpringParams::from_damping_ratio(150.0, 0.45),
                                        );
                                } else {
                                    this.status_label = "SPRING SNAP TO DOCK";

                                    this.marble_x.interrupt_spring(
                                        HOME_X,
                                        SpringParams::from_damping_ratio(180.0, 0.55),
                                    );
                                    this.marble_y.interrupt_spring(
                                        HOME_Y,
                                        SpringParams::from_damping_ratio(180.0, 0.55),
                                    );
                                }

                                cx.notify();
                            }
                        }),
                    )
                    // -------------------------------------------------------------
                    // ANIMOTION CLIP PIPELINE
                    // -------------------------------------------------------------
                    .animotion_clip(
                        ElementId::Name("interactive_canvas_clip".into()),
                        move |c| {
                            c.attach(&marble_x);
                            c.attach(&marble_y);
                            c.attach(&marble_color);
                            (marble_x, marble_y, marble_color)
                        },
                        move |el, (mx, my, mcol)| {
                            let mx = mx.clone();
                            let my = my.clone();
                            let mcol = mcol.clone();
                            let arena_origin = Arc::clone(&arena_origin);

                            el.child(
                                canvas(
                                    |_, _, _| {},
                                    move |bounds, _, window, _| {
                                        *arena_origin.lock().unwrap() = bounds.origin;

                                        let cur_x = mx.get();
                                        let cur_y = my.get();
                                        let active_color = mcol.get();

                                        let home_center_x = bounds.origin.x + px(HOME_X + MARBLE_SIZE / 2.0);
                                        let home_center_y = bounds.origin.y + px(HOME_Y + MARBLE_SIZE / 2.0);
                                        let marble_center_x = bounds.origin.x + px(cur_x + MARBLE_SIZE / 2.0);
                                        let marble_center_y = bounds.origin.y + px(cur_y + MARBLE_SIZE / 2.0);

                                        // 1. HOME DOCK TARGET RING
                                        let mut dock_builder = PathBuilder::fill();
                                        for i in 0..32 {
                                            let theta = (i as f32 / 32.0) * 2.0 * PI;
                                            let pt = point(
                                                home_center_x + px(36.0 * theta.cos()),
                                                home_center_y + px(36.0 * theta.sin()),
                                            );
                                            if i == 0 {
                                                dock_builder.move_to(pt);
                                            } else {
                                                dock_builder.line_to(pt);
                                            }
                                        }
                                        if let Ok(path) = dock_builder.build() {
                                            window.paint_path(path, hsla(0.48, 0.50, 0.40, 0.15));
                                        }

                                        // 2. ELASTIC TETHER CORD
                                        if is_dragging {
                                            let mut cord_builder = PathBuilder::fill();
                                            let dx = cur_x - HOME_X;
                                            let dy = cur_y - HOME_Y;
                                            let len = (dx * dx + dy * dy).sqrt().max(1.0);
                                            let nx = -dy / len * 2.0;
                                            let ny = dx / len * 2.0;

                                            cord_builder.move_to(point(home_center_x + px(nx), home_center_y + px(ny)));
                                            cord_builder.line_to(point(marble_center_x + px(nx), marble_center_y + px(ny)));
                                            cord_builder.line_to(point(marble_center_x - px(nx), marble_center_y - px(ny)));
                                            cord_builder.line_to(point(home_center_x - px(nx), home_center_y - px(ny)));

                                            if let Ok(path) = cord_builder.build() {
                                                window.paint_path(path, hsla(0.48, 0.90, 0.60, 0.40));
                                            }
                                        }

                                        // 3. CONTACT FLOOR SHADOW
                                        let shadow_center_x = marble_center_x;
                                        let shadow_center_y = marble_center_y + px(MARBLE_SIZE * 0.48);
                                        let mut shadow_builder = PathBuilder::fill();
                                        for i in 0..24 {
                                            let theta = (i as f32 / 24.0) * 2.0 * PI;
                                            let pt = point(
                                                shadow_center_x + px(MARBLE_SIZE * 0.42 * theta.cos()),
                                                shadow_center_y + px(MARBLE_SIZE * 0.14 * theta.sin()),
                                            );
                                            if i == 0 {
                                                shadow_builder.move_to(pt);
                                            } else {
                                                shadow_builder.line_to(pt);
                                            }
                                        }
                                        if let Ok(path) = shadow_builder.build() {
                                            window.paint_path(path, hsla(0.65, 0.40, 0.04, 0.45));
                                        }

                                        // 4. AMBIENT GLOW & SPHERE
                                        let marble_origin = point(bounds.origin.x + px(cur_x), bounds.origin.y + px(cur_y));

                                        window.paint_quad(
                                            fill(
                                                Bounds {
                                                    origin: marble_origin - point(px(4.0), px(4.0)),
                                                    size: size(px(MARBLE_SIZE + 8.0), px(MARBLE_SIZE + 8.0)),
                                                },
                                                hsla(active_color.h, active_color.s, active_color.l, 0.30),
                                            )
                                            .corner_radii(Corners::all(px((MARBLE_SIZE + 8.0) / 2.0))),
                                        );

                                        window.paint_quad(
                                            fill(
                                                Bounds {
                                                    origin: marble_origin,
                                                    size: size(px(MARBLE_SIZE), px(MARBLE_SIZE)),
                                                },
                                                active_color,
                                            )
                                            .corner_radii(Corners::all(px(MARBLE_SIZE / 2.0))),
                                        );

                                        // 5. RIM LIGHT & SPECULAR HIGHLIGHT
                                        let rim_cx = marble_origin.x + px(MARBLE_SIZE * 0.50);
                                        let rim_cy = marble_origin.y + px(MARBLE_SIZE * 0.82);
                                        let mut rim_builder = PathBuilder::fill();
                                        for i in 0..24 {
                                            let theta = (i as f32 / 24.0) * 2.0 * PI;
                                            let pt = point(
                                                rim_cx + px(MARBLE_SIZE * 0.24 * theta.cos()),
                                                rim_cy + px(MARBLE_SIZE * 0.08 * theta.sin()),
                                            );
                                            if i == 0 {
                                                rim_builder.move_to(pt);
                                            } else {
                                                rim_builder.line_to(pt);
                                            }
                                        }
                                        if let Ok(path) = rim_builder.build() {
                                            window.paint_path(path, hsla(active_color.h, 0.5, 0.90, 0.40));
                                        }

                                        let spec_cx = marble_origin.x + px(MARBLE_SIZE * 0.28);
                                        let spec_cy = marble_origin.y + px(MARBLE_SIZE * 0.28);
                                        let spec_rx = MARBLE_SIZE * 0.16;
                                        let spec_ry = MARBLE_SIZE * 0.09;
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
                                            window.paint_path(path, hsla(0.0, 0.0, 1.0, 0.90));
                                        }
                                    },
                                )
                                .size_full(),
                            )
                        },
                    )
                    // -------------------------------------------------------------
                    // HUD TELEMETRY OVERLAYS
                    // -------------------------------------------------------------
                    .child(
                        div()
                            .absolute()
                            .top(px(20.0))
                            .left(px(24.0))
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(hsla(0.48, 0.95, 0.70, 1.0))
                                    .child("Kinetic Throw & Spring Snap"),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(hsla(0.65, 0.15, 0.65, 1.0))
                                    .child(format!("State: {}", status)),
                            ),
                    )
                    .child(
                        div()
                            .absolute()
                            .top(px(20.0))
                            .right(px(24.0))
                            .flex()
                            .flex_col()
                            .items_end()
                            .gap_1()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(hsla(0.12, 0.95, 0.65, 1.0))
                                    .child(format!(
                                        "Exit Velocity: |v| = {:.0} px/s",
                                        (last_vel.0.powi(2) + last_vel.1.powi(2)).sqrt()
                                    )),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(hsla(0.65, 0.15, 0.55, 1.0))
                                    .child(format!("Vx: {:.0} | Vy: {:.0}", last_vel.0, last_vel.1)),
                            ),
                    ),
            )
    }
}

fn main() {
    shared::run("Interactive Velocity Throw & Snap", |_, _| {
        InteractiveThrowView::new()
    });
}
