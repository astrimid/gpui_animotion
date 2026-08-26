// examples/hover_card.rs
use gpui::*;
use gpui_motion::*;

struct HoverCardDemo;

impl Render for HoverCardDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(hsla(0.6, 0.1, 0.05, 1.0))
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .relative()
                    .w(px(280.0))
                    .h(px(360.0))
                    .rounded_2xl()
                    .border_1()
                    .border_color(hsla(0.0, 0.0, 1.0, 0.1))
                    .p_6()
                    .flex()
                    .flex_col()
                    .justify_between()
                    .animotion("card_container", all((
                        // Pulsing background gradient feel
                        prop(
                            tween(hsla(0.65, 0.4, 0.12, 1.0), hsla(0.75, 0.5, 0.18, 1.0), 2.0)
                                .tween(hsla(0.65, 0.4, 0.12, 1.0), 2.0),
                            |el, color| el.bg(color),
                        ),
                        // Breathing size scale simulation
                        prop(
                            tween(280.0, 292.0, 1.5).tween(280.0, 1.5),
                            |el, w| el.w(px(w)),
                        ),
                        // Subtle vertical float
                        prop(
                            tween(0.0, -12.0, 1.8).tween(0.0, 1.8),
                            |el, y| el.top(px(y)),
                        ),
                    )))
                    .child(
                        div()
                            .size_12()
                            .rounded_lg()
                            .animotion("card_icon", all((
                                prop(
                                    tween(hsla(0.55, 0.9, 0.6, 1.0), hsla(0.85, 0.9, 0.6, 1.0), 2.0)
                                        .tween(hsla(0.55, 0.9, 0.6, 1.0), 2.0),
                                    |el, c| el.bg(c),
                                ),
                            )))
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(div().h(px(12.0)).w(px(140.0)).rounded_md().bg(hsla(0.0, 0.0, 1.0, 0.8)))
                            .child(div().h(px(8.0)).w(px(200.0)).rounded_md().bg(hsla(0.0, 0.0, 1.0, 0.3)))
                    )
            )
    }
}
