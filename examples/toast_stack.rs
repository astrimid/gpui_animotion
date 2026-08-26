mod shared;

use gpui::*;
use gpui_motion::*;

struct ToastStackView;

impl Render for ToastStackView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(hsla(0.0, 0.0, 0.07, 1.0))
            .flex()
            .justify_center()
            .p_8()
            .child(
                div()
                    .w(px(340.0))
                    .h(px(64.0))
                    .rounded_xl()
                    .p_4()
                    .flex()
                    .items_center()
                    .justify_between()
                    .border_1()
                    .border_color(hsla(0.0, 0.0, 1.0, 0.1))
                    .motion(
                        "toast_banner",
                        all((
                            // Slide down from top, hold, and retract
                            prop(
                                tween(-80.0, 20.0, 0.4)
                                    .tween(20.0, 2.0)
                                    .tween(-80.0, 0.4),
                                |el, y| el.top(px(y)),
                            ),
                            // Background glow state transition
                            prop(
                                tween(hsla(0.38, 0.6, 0.15, 1.0), hsla(0.38, 0.6, 0.2, 1.0), 1.4)
                                    .tween(hsla(0.38, 0.6, 0.15, 1.0), 1.4),
                                |el, c| el.bg(c),
                            ),
                        )),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .child(
                                div()
                                    .size_3()
                                    .rounded_full()
                                    .bg(hsla(0.38, 0.8, 0.55, 1.0)),
                            )
                            .child("Deployment successful"),
                    ),
            )
    }
}

fn main() {
    shared::run("Notification Toast Stack", |_, _| ToastStackView);
}
