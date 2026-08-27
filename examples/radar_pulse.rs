mod shared;

use gpui::*;
use gpui_animotion::*;

struct RadarPulseView;

impl Render for RadarPulseView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let ring = |delay: f32| {
            all((
                // Expansion loop
                prop(
                    tween(20.0, 20.0, delay)
                        .tween(180.0, 2.0)
                        .tween(20.0, 0.0),
                    |el, size| el.size(px(size)),
                ),
                // Color & Opacity pulse
                prop(
                    tween(hsla(0.45, 0.9, 0.5, 0.8), hsla(0.45, 0.9, 0.5, 0.8), delay)
                        .tween(hsla(0.45, 0.9, 0.5, 0.0), 2.0)
                        .tween(hsla(0.45, 0.9, 0.5, 0.8), 0.0),
                    |el, color| el.bg(color),
                ),
            ))
        };

        div()
            .size_full()
            .bg(hsla(0.0, 0.0, 0.03, 1.0))
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .relative()
                    .size(px(200.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .absolute()
                            .rounded_full()
                            .animotion("pulse_1", ring(0.0)),
                    )
                    .child(
                        div()
                            .absolute()
                            .rounded_full()
                            .animotion("pulse_2", ring(0.6)),
                    )
                    .child(
                        div()
                            .absolute()
                            .rounded_full()
                            .animotion("pulse_3", ring(1.2)),
                    )
                    .child(
                        // Center Core Node
                        div()
                            .size_6()
                            .rounded_full()
                            .bg(hsla(0.45, 0.9, 0.6, 1.0)),
                    ),
            )
    }
}

fn main() {
    shared::run("Radar Pulse Scanner", |_, _| RadarPulseView);
}
