# gpui_animotion

[![Crates.io](https://img.shields.io/crates/v/gpui_animotion.svg)](https://crates.io/crates/gpui_animotion)
[![Documentation](https://docs.rs/gpui_animotion/badge.svg)](https://docs.rs/gpui_animotion)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

A declarative, combinator-driven procedural animation engine for [GPUI](https://github.com/zed-industries/zed).

Inspired by the procedural composition of tools like **Motion Canvas** and the timing orchestration of **GSAP**, `gpui_animotion` brings fine-grained timeline control and physical simulations directly to native Rust UI components.

## Overview & Paradigm

UI animation libraries generally fall into two categories:

1. **Implicit Layout Transitions:** Optimized for $`A \to B`$ state changes, but cumbersome when orchestrating multi-element scenes with strict timing dependencies.
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
gpui_animotion = "0.3"
```

## Quickstart

Clone the repository and run the included example:

```bash
git clone https://github.com/astrimid/gpui_animotion.git
cd gpui_animotion
cargo run --example many_shiny_balls
```

## Usage Examples

### 1. Spring & Tween Inline Animation (`.animotion`)

For straightforward UI components, use `.animotion()` to declaratively map tweens directly onto standard `Div` attributes like position and color:

```rust
use gpui::*;
use gpui_animotion::*;

fn render_spring_box() -> impl IntoElement {
    div()
        .absolute()
        .size_12()
        .rounded_lg()
        .bg(rgb(0x3b82f6))
        .animotion(
            "spring_box",
            all((
                // X-axis: Analytical spring to 200px, then smooth tween back to 0px
                prop(
                    spring(0.0, 200.0, SpringParams::from_damping_ratio(250.0, 0.55))
                        .tween(0.0, 0.6),
                    |el, x| el.left(px(x)),
                ),
                // Concurrently pulse scale/opacity
                prop(
                    tween(0.8, 1.1, 0.4).tween(0.8, 0.6),
                    |el, s| el.opacity(s),
                ),
            )),
        )
}
```

### 2. Multi-Body Physics & Custom Canvas (`.animotion_clip`)

For complex scenes (like multi-ball physics simulations), use `.animotion_clip()` to declare tracked properties, handle physics bindings, and render via canvas:

```rust
use gpui::*;
use gpui_animotion::*;

#[derive(Clone)]
struct BallProps {
    x: f32,
    y: Prop<f32>,
    color: Prop<Hsla>,
}

fn render_spring_marbles(floor_y: f32) -> impl IntoElement {
    div().animotion_clip(
        ElementId::Name("physics_spring_marbles".into()),
        |c| vec![
            // 1. Lively underdamped spring (low damping ratio = visible oscillation)
            BallProps {
                x: 100.0,
                y: c.prop(40.0)
                    .spring(floor_y, SpringParams::from_damping_ratio(220.0, 0.22))
                    .spring(40.0, SpringParams::from_damping_ratio(180.0, 0.60))
                    .clone(),
                color: c.prop(hsla(0.75, 0.8, 0.50, 1.0)),
            },
            // 2. Snappy UI spring (medium damping ratio = smooth settle)
            BallProps {
                x: 260.0,
                y: c.prop(80.0)
                    .spring(floor_y, SpringParams::from_damping_ratio(180.0, 0.50))
                    .spring(80.0, SpringParams::from_damping_ratio(180.0, 0.70))
                    .clone(),
                color: c.prop(hsla(0.25, 0.8, 0.45, 1.0)),
            },
        ],
        move |el, balls| {
            let balls = balls.to_vec();
            el.child(
                canvas(
                    |_, _, _| {},
                    move |bounds, _, window, _| {
                        for ball in &balls {
                            let current_y = ball.y.get();
                            let origin = point(
                                bounds.origin.x + px(ball.x),
                                bounds.origin.y + px(current_y),
                            );

                            window.paint_quad(
                                fill(
                                    Bounds {
                                        origin,
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

The engine is built around composable building blocks for timing, interpolation, and physics simulation:

### 1. Keyframing & Tweens (`tween`)

Tweens form the baseline of programmatic transitions. A tween linearly or smoothly interpolates a property value from its current state to a target value over a specified duration in seconds:

* **Sequential Chaining**: Chain multiple `.tween()` calls together to build multi-step paths (`tween(0.0, 100.0, 0.5).tween(50.0, 0.3)`).
* **Native Type Support**: Interpolation is implemented automatically for f32, Hsla, Rgba, and other GPUI geometry primitives.
* (TODO: Add Easing) **Easing**:

### 2. Analytical Spring Physics (`spring`)

Spring tracks compute closed-form harmonic oscillator equations at discrete sampling steps (`sample_fps`), providing high-performance physical motion without numeric accumulation error:

* **`SpringParams`:** Configure mass ($m$), stiffness ($k$), damping ($c$), initial velocity ($v_0$), and settling threshold ($\epsilon$).
* **Damping Ratio Helper:** Use `SpringParams::from_damping_ratio(stiffness, zeta)` to configure response behavior quickly:
* $\zeta < 1.0$: **Underdamped** (elastic bounce with visible oscillation).
* $\zeta = 1.0$: **Critically Damped** (fastest possible settlement without overshoot).
* $\zeta > 1.0$: **Overdamped** (gentle, sluggish deceleration).


* **Automatic Duration Detection:** Automatically calculates resting duration based on settling thresholds, or accepts explicit duration overrides.

```rust
// Chain a spring launch into a return bounce
prop.spring(300.0, SpringParams::from_damping_ratio(200.0, 0.35))
    .spring(0.0, SpringParams::from_damping_ratio(180.0, 0.70));

```

### 3. Gravity & Kinematic Trajectories (`gravity`)

* **`gravity(...)`**: Simulates constant acceleration, floor collision detection, and velocity decay based on a coefficient of restitution.
* Eliminates the need to hand-craft bounce parabola keyframes.

### 4. Property Handles (`Prop<T>`)

`Prop<T>` manages animated state across frames:

* **Thread-Safe Sampling:** Backed by thread-safe synchronization primitives (`Arc<Mutex<T>>`), allowing properties to be sampled with near-zero overhead during GPUI paint passes.
* **Fluent Builder:** Attach `.tween()`, `.spring()`, and `.gravity()` calls directly to the property builder before mounting into the view tree.

### 5. Combinators (`all` and `seq`)

Orchestrate how multiple properties or elements execute:

* **`all(...)`**: Executes wrapped property tracks in parallel concurrently.
* **`seq(...)`**: Executes wrapped animation tracks sequentially in series *(coming soon)*.

### 6. Continuous Looping

Animations loop seamlessly when a sequence is terminated by returning to the initial value. Chaining a physics track with an inverse spring or return `.tween()` guarantees visual continuity across continuous playback cycles.

## License

Dual-licensed under either of:

* Apache License, Version 2.0 ([LICENSE-APACHE](http://www.apache.org/licenses/LICENSE-2.0))
* MIT license ([LICENSE-MIT](http://opensource.org/licenses/MIT))
