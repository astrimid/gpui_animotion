use gpui::*;

/// Shared window runner for `gpui_motion` examples.
///
/// Handles application initialization, window sizing, centered placement,
/// and view mounting with minimal boilerplate.
pub fn run<V: Render + 'static>(
    title: &'static str,
    build_view: impl FnOnce(&mut Window, &mut Context<V>) -> V + 'static,
) {
    gpui_platform::application().run(move |cx: &mut App| {
        let bounds = Bounds {
            origin: Point::new(px(160.0), px(120.0)),
            size: size(px(960.0), px(640.0)),
        };

        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(TitlebarOptions {
                title: Some(title.into()),
                appears_transparent: true,
                traffic_light_position: Some(Point::new(px(12.0), px(12.0))),
            }),
            ..Default::default()
        };

        cx.open_window(options, move |window, cx| cx.new(|cx| build_view(window, cx)))
            .unwrap();
    });
}
