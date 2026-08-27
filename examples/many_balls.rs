mod shared;

use gpui::*;
use gpui_animotion::*;

struct BouncingBallView;

impl Render for BouncingBallView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(hsla(0.0, 0.0, 0.08, 1.0))
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .relative()
                    .w(px(600.0))
                    .h(px(400.0))
                    .bg(hsla(0.0, 0.0, 0.12, 1.0))
                    .rounded_xl()
                    .animotion_clip(
                        ElementId::Name("bouncing_balls".into()),
                        |c| {
                            vec![
                                (
                                    100.0,
                                    c.prop(40.0).tween(300.0, 0.8).tween(40.0, 0.8).clone(),
                                    c.prop(hsla(0.55, 0.8, 0.55, 1.0))
                                        .tween(hsla(0.95, 0.8, 0.55, 1.0), 1.6)
                                        .tween(hsla(0.55, 0.8, 0.55, 1.0), 1.6)
                                        .clone(),
                                    c.prop(60.0).tween(76.0, 0.1).tween(60.0, 0.8).clone(),
                                ),
                                (
                                    260.0,
                                    c.prop(40.0).tween(300.0, 1.1).tween(40.0, 1.1).clone(),
                                    c.prop(hsla(0.35, 0.8, 0.55, 1.0))
                                        .tween(hsla(0.15, 0.8, 0.55, 1.0), 2.2)
                                        .tween(hsla(0.35, 0.8, 0.55, 1.0), 2.2)
                                        .clone(),
                                    c.prop(60.0).tween(76.0, 0.15).tween(60.0, 1.1).clone(),
                                ),
                                (
                                    420.0,
                                    c.prop(40.0).tween(300.0, 0.6).tween(40.0, 0.6).clone(),
                                    c.prop(hsla(0.75, 0.8, 0.55, 1.0))
                                        .tween(hsla(0.45, 0.8, 0.55, 1.0), 1.2)
                                        .tween(hsla(0.75, 0.8, 0.55, 1.0), 1.2)
                                        .clone(),
                                    c.prop(60.0).tween(76.0, 0.08).tween(60.0, 0.6).clone(),
                                ),
                            ]
                        },
                        move |el, props| {
                            let props = props.clone();
                            el.child(
                                canvas(
                                    |_, _, _| {},
                                    move |bounds, _, window, _| {
                                        for (x, y, color, sz) in &props {
                                            let s = sz.get();
                                            window.paint_quad(
                                                fill(
                                                    Bounds {
                                                        origin: point(
                                                            bounds.origin.x + px(*x),
                                                            bounds.origin.y + px(y.get()),
                                                        ),
                                                        size: size(px(s), px(s)),
                                                    },
                                                    color.get(),
                                                )
                                                .corner_radii(Corners::all(px(s / 2.0))),
                                            );
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
    shared::run("Multiple Bouncing Balls Canvas Example", |_, _| BouncingBallView);
}
