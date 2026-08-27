# gpui_animotion

[![Crates.io](https://img.shields.io/crates/v/gpui_animotion.svg)](https://crates.io/crates/gpui_animotion)
[![Documentation](https://docs.rs/gpui_animotion/badge.svg)](https://docs.rs/gpui_animotion)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

A declarative, combinator-driven procedural animation engine for [GPUI](https://github.com/zed-industries/zed).

Inspired by the procedural composition of tools like **Motion Canvas** and the timing orchestration of **GSAP**, `gpui_animotion` brings fine-grained timeline control and physical simulations directly to native Rust UI components.

## Overview & Paradigm

UI animation libraries generally fall into two categories:

1. **Implicit Layout Transitions:** Optimized for $A \to B$ state changes, but cumbersome when orchestrating multi-element scenes with strict timing dependencies.
2. **Keyframe Timelines:** Highly controllable, but often decoupled from modern component-driven layout trees.

`gpui_animotion` bridges this gap for GPUI by introducing **combinator-driven element animation**. Instead of managing raw frame ticks or maintaining imperative timeline controllers, you attach composable timing pipelines directly to standard GPUI elements using reactive property mappers and fluent animation combinators.

### Key Features

* **Direct Element Binding:** Animate standard GPUI layout properties, custom canvas properties, and reactive variables directly.
* **Procedural Physics:** Built-in simulation tools like realistic gravity and bouncing trajectories.
* **Fluent Chaining:** Easily compose complex sequences combining keyframe `.tween()` transitions and physics algorithms.
* **Zero Canvas Lock-In:** Works seamlessly with standard GPUI views and layouts.

## Installation

Add `gpui_animotion` and `gpui` to your `Cargo.toml`:

```toml
[dependencies]
gpui = { package = "gpui-unofficial", version = "1.16" }
gpui_animotion = "0.2"
```

## Quickstart

Clone the repository and run the included example:

```bash
git clone https://github.com/astrimid/gpui_animotion.git
cd gpui_animotion
cargo run --example many_shiny_balls
```

## Usage Examples

### 1. Simple Inline Animation (`.animotion`)

For straightforward UI components, use `.animotion()` to declaratively map tweens directly onto standard `Div` attributes like position and color:

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

### 2. Advanced Physics & Clips (`.animotion_clip`)

For complex scenes (like multi-ball physics simulations), use `.animotion_clip()` to declare tracked properties, handle physics bindings, and render via canvas:

```rust
use gpui::*;
use gpui_animotion::*;

#[derive(Clone)]
struct BallProps {
    y: Prop<f32>,
    color: Prop<Hsla>,
}

fn render_physics_marbles(floor_y: f32) -> impl IntoElement {
    div().animotion_clip(
        ElementId::Name("physics_ball".into()),
        |c| vec![
            BallProps {
                // Fluent chain: start at 40.0, apply gravity bounce, then loop back to top smoothly
                y: c.prop(40.0)
                    .gravity(
                        GravityParams {
                            gravity: 2400.0,
                            restitution: 0.80,
                            floor_y,
                            ..Default::default()
                        },
                        6,
                    )
                    .tween(40.0, 1.0)
                    .clone(),
                color: c.prop(hsla(0.75, 0.8, 0.50, 1.0)),
            }
        ],
        move |el, balls| {
            let balls = balls.to_vec();
            el.child(
                canvas(
                    |_, _, _| {},
                    move |bounds, _, window, _| {
                        for ball in &balls {
                            let current_y = ball.y.get();
                            let ball_origin = point(
                                bounds.origin.x + px(100.0),
                                bounds.origin.y + px(current_y),
                            );

                            window.paint_quad(
                                fill(
                                    Bounds {
                                        origin: ball_origin,
                                        size: size(px(60.0), px(60.0)),
                                    },
                                    ball.color.get(),
                                )
                                .corner_radii(Corners::all(px(30.0))),
                            );
                        }
                    },
                )
                .size_full(),
            )
        },
    )
}
```

## Core Concepts

The engine is built around a few primary building blocks that handle timing, interpolation, and state reactivity:

### 1. `tween` and Keyframing

Tweens form the baseline of all procedural transitions. A tween interpolates a property value from its current state to a specified target over a set duration (measured in seconds).

* **Sequential Chaining**: You can chain multiple `.tween()` calls together to build complex multi-step timelines (e.g., scale up, hold, then scale down).
* **Easing & Interpolation**: Handled automatically via trait implementations for native types.

### 2. Physics & Procedural Tracks (`gravity`)

Beyond standard keyframe tweens, `gpui_animotion` supports programmatic physical simulations:

* **`gravity(...)`**: Automatically calculates realistic acceleration, velocity loss, and energy restitution upon hitting a specified floor boundary.
* **Custom Trajectories**: Useful for natural movement effects without needing to hand-craft every single keyframe manually.

### 3. Property Management (`Prop<T>`)

A `Prop<T>` manages the underlying state of an animated value across frames.

* **Thread-Safe Handles**: Backed by thread-safe synchronization primitives (`Arc<Mutex<T>>`), allowing properties to be sampled efficiently inside high-frequency render loops.
* **Fluent API**: Allows mixing procedural physics and manual tweens directly on the property builder before mounting.

### 4. Combinators (`all` and `seq`)

Orchestrate how multiple properties or elements execute relative to one another:

* **`all(...)`**: Executes wrapped property animations and tracks concurrently in parallel.
* **`seq(...)`**: Executes wrapped animations sequentially one after another *(coming soon)*.

### 5. Continuous Looping

To prevent animations from popping or resetting abruptly, sequences can be designed to loop seamlessly. By terminating a physics or keyframe track with an invisible return `.tween()` back to the initial starting value, elements can be smoothly guided back to their origin without breaking visual continuity.

## License

Dual-licensed under either of:

* Apache License, Version 2.0 ([LICENSE-APACHE](http://www.apache.org/licenses/LICENSE-2.0))
* MIT license ([LICENSE-MIT](http://opensource.org/licenses/MIT))
