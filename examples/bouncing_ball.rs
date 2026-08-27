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
                    .child(
                        div()
                            .absolute()
                            .rounded_full()
                            .animotion("bouncing_ball", all((
                                prop(
                                    tween(40.0, 300.0, 0.8).tween(40.0, 0.8),
                                    |el, y| el.top(px(y)),
                                ),
                                prop(
                                    tween(40.0, 500.0, 1.6).tween(40.0, 1.6),
                                    |el, x| el.left(px(x)),
                                ),
                                prop(
                                    tween(hsla(0.55, 0.8, 0.55, 1.0), hsla(0.95, 0.8, 0.55, 1.0), 1.6)
                                        .tween(hsla(0.55, 0.8, 0.55, 1.0), 1.6),
                                    |el, color| el.bg(color),
                                ),
                                prop(
                                    tween(60.0, 60.0, 0.7)
                                        .tween(76.0, 0.1)
                                        .tween(60.0, 0.8),
                                    |el, s| el.size(px(s)),
                                ),
                            ))),
                    ),
            )
    }
}

fn main() {
    shared::run("Bouncing Ball Example", |_, _| BouncingBallView);
}

//fn main() {
//    gpui_platform::application().run(move |cx: &mut App| {
//        let options = WindowOptions {
//            window_bounds: Some(WindowBounds::Windowed(Bounds {
//                origin: Point::new(px(200.0), px(200.0)),
//                size: size(px(1000.0), px(800.0)),
//            })),
//            ..Default::default()
//        };
//
//        cx.open_window(options, move |_window, cx| {
//            cx.new(|_| MotionDemoView)
//        })
//        .unwrap();
//    });
//}
