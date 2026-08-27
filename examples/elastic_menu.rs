mod shared;

use gpui::*;
use gpui_animotion::*;

struct ElasticMenuView;

impl Render for ElasticMenuView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let menu_items = ["Dashboard", "Analytics", "Projects", "Settings"];

        div()
            .size_full()
            .bg(hsla(0.65, 0.2, 0.06, 1.0))
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .w(px(240.0))
                    .p_3()
                    .rounded_2xl()
                    .bg(hsla(0.65, 0.2, 0.1, 1.0))
                    .border_1()
                    .border_color(hsla(0.0, 0.0, 1.0, 0.08))
                    .flex()
                    .flex_col()
                    .gap_2()
                    .children(menu_items.into_iter().enumerate().map(|(idx, label)| {
                        let delay = idx as f32 * 0.15;
                        div()
                            .h(px(40.0))
                            .px_4()
                            .rounded_xl()
                            .flex()
                            .items_center()
                            .child(label)
                            .animotion(
                                label,
                                all((
                                    // Slide in from left with staggered entrance
                                    prop(
                                        tween(-40.0, -40.0, delay)
                                            .tween(0.0, 0.5),
                                        |el, x| el.left(px(x)),
                                    ),
                                    // Soft opacity fade-in
                                    prop(
                                        tween(0.2, 0.2, delay)
                                            .tween(1.0, 0.5),
                                        |el, opacity| el.bg(hsla(0.0, 0.0, 1.0, opacity * 0.05)),
                                    ),
                                )),
                            )
                    })),
            )
    }
}

fn main() {
    shared::run("Elastic Navigation Menu", |_, _| ElasticMenuView);
}
