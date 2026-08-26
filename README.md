# gpui_animotion

[![Crates.io](https://img.shields.io/crates/v/gpui_animotion.svg)](https://crates.io/crates/gpui_animotion)
[![Documentation](https://docs.rs/gpui_animotion/badge.svg)](https://docs.rs/gpui_animotion)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

A declarative, combinator-driven procedural animation engine for [GPUI](https://github.com/zed-industries/zed).

Inspired by the procedural composition of tools like **Motion Canvas** and the timing orchestration of **GSAP**, `gpui_animotion` brings fine-grained timeline control directly to native Rust UI components.

## Overview & Paradigm

UI animation libraries generally fall into two categories:

1. **Implicit Layout Transitions:** Optimized for $A \to B$ state changes, but cumbersome when orchestrating multi-element scenes with strict timing dependencies.
2. **Keyframe Timelines:** Highly controllable, but often decoupled from modern component-driven layout trees.

`gpui_animotion` bridges this gap for GPUI by introducing **combinator-driven element animation**. Instead of managing raw frame ticks or maintaining imperative timeline controllers, you attach composable timing pipelines directly to standard GPUI `Div` elements using reactive property mappers and parallel or sequential combinators.

### Key Features

* **Direct Element Binding:** Animate standard GPUI layout properties (`.top()`, `.bg()`, `.opacity()`, etc.) directly on `Div` instances.
* **Combinator Composition:** Combine interpolations concurrently with `all(...)` or sequentially with `seq(...)`.
* **Native Interpolation:** Built-in interpolators for native GPUI types including `Length`, `Hsla`, `f32`, `Pixels`, and colors.
* **Zero Canvas Lock-In:** Works with standard GPUI views and elements—no special isolated 2D canvas context required.

## Installation

Add `gpui_animotion` and `gpui` to your `Cargo.toml`:

```toml
[dependencies]
gpui = { package = "gpui-unofficial", version = "1.16" }
gpui_animotion = "0.1"
```

## Quickstart

Clone the repository and run the included example:

```bash
git clone [https://github.com/astrimid/gpui_animotion.git](https://github.com/astrimid/gpui_animotion.git)
cd gpui_motion
cargo run --example bouncing_ball
```

## Basic Usage

Attach `.animotion()` to any GPUI `Div` in your `render` method:

```rust
use gpui::*;
use gpui_animotion::*;

fn render_bouncing_ball() -> impl IntoElement {
    div()
        .absolute()
        .size_12()
        .rounded_full()
        .animotion(
            "bouncing_ball",
            all((
                // Y-axis translation: Bounce down and back up
                prop(
                    tween(40.0, 300.0, 0.8).tween(40.0, 0.8),
                    |el, y| el.top(px(y)),
                ),
                // Background color transition run concurrently
                prop(
                    tween(hsla(0.55, 0.8, 0.55, 1.0), hsla(0.95, 0.8, 0.55, 1.0), 1.6)
                        .tween(hsla(0.55, 0.8, 0.55, 1.0), 1.6),
                    |el, color| el.bg(color),
                ),
            )),
        )
}

```

## Core Concepts

### 1. `tween`

Defines an interpolation from a start value to an end value over a given duration (in seconds). Tweens can be chained together sequentially:

```rust
// Interpolates from 0.0 to 100.0 over 0.5s, then from 100.0 to 50.0 over 0.3s
tween(0.0, 100.0, 0.5).tween(50.0, 0.3)
```

### 2. `prop`

Maps a `tween` or timeline structure to a specific GPUI element modifier:

```rust
prop(
    tween(0.0, 1.0, 0.4),
    |el, opacity| el.opacity(opacity),
)
```

### 3. `all` and `seq`

Orchestrate execution flow across properties or elements:

* **`all(...)`**: Executes wrapped animations in **parallel**.
* TODO **`seq(...)`**: Executes wrapped animations **sequentially**.

```rust
// Run two property animations simultaneously
all((
    prop(tween(0.0, 100.0, 0.5), |el, x| el.left(px(x))),
    prop(tween(1.0, 2.0, 0.5), |el, s| el.scale(s)),
))

```

## License

Dual-licensed under either of:

* Apache License, Version 2.0 ([LICENSE-APACHE](http://www.apache.org/licenses/LICENSE-2.0))
* MIT license ([LICENSE-MIT](http://opensource.org/licenses/MIT))

