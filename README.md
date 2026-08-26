# gpui_motion

A procedural animation engine for [GPUI](https://github.com/zed-industries/zed/tree/main/crates/gpui), inspired by [Canvas Commons / Motion Canvas](https://canvascommons.io/).

## Quickstart

```bash
git clone git@github.com:astrimid/gpui_motion.git
cd gpui_motion
cargo run --example bouncing_ball
```

## Usage

Add `gpui_motion` to your dependencies and apply `.motion()` to any `Div`:

```rust
use gpui::*;
use gpui_motion::*;

/// in render():
div()
    .absolute()
    .rounded_full()
    .motion("bouncing_ball", all((
        prop(
            tween(40.0, 300.0, 0.8).tween(40.0, 0.8),
            |el, y| el.top(px(y)),
        ),
        prop(
            tween(hsla(0.55, 0.8, 0.55, 1.0), hsla(0.95, 0.8, 0.55, 1.0), 1.6)
                .tween(hsla(0.55, 0.8, 0.55, 1.0), 1.6),
            |el, color| el.bg(color),
        ),
    )))

```

## License

MIT OR Apache-2.0
