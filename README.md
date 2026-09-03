# gpui_animotion

[![Crates.io](https://img.shields.io/crates/v/gpui_animotion.svg)](https://crates.io/crates/gpui_animotion)
[![Documentation](https://docs.rs/gpui_animotion/badge.svg)](https://docs.rs/gpui_animotion)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

A declarative, combinator-driven procedural animation engine for [GPUI](https://github.com/zed-industries/zed).

Inspired by the procedural composition of tools like **Motion Canvas** and the timing orchestration of **GSAP**, `gpui_animotion` brings fine-grained timeline control and physical simulations directly to native Rust UI components.

## Overview & Paradigm

UI animation approaches generally fall into three paradigms, each with distinct tradeoffs:

1. **Implicit Layout Transitions:** Optimized for basic $A \to B$ state changes, but cumbersome when orchestrating multi-element scenes with precise timing dependencies.
2. **Keyframe Timelines:** Highly controllable, but often imperative, verbose, and decoupled from modern reactive layout trees.
3. **Force-Based Game Physics:** Expressive and interactive, but frame-rate dependent, prone to overshoot under CPU jitter, and difficult to synchronize deterministically.

`gpui_animotion` bridges these paradigms for GPUI by pairing **closed-form analytical segment pipelines** with **fluent, combinator-driven element animation**.

```
┌─────────────────────────────────────────────────────────────┐
│                    Track<T> Pipeline                        │
├─────────────────┬───────────────────┬───────────────────────┤
│ Segment 1:      │ Segment 2:        │ Segment 3:            │
│ TweenSegment    │ SpringSegment     │ FlickSegment          │
│ duration: 0.4s  │ duration: 0.8s    │ duration: 1.1s        │
└────────┬────────┴─────────┬─────────┴───────────┬───────────┘
         │                  │                     │
         ▼                  ▼                     ▼
   evaluate(t)        evaluate(t)           evaluate(t)
   velocity(t)        velocity(t)           velocity(t)

```

Instead of maintaining imperative timeline controllers, pre-baking discrete keyframe arrays, or running frame-by-frame numerical integration loops:

* **$\mathcal{O}(1)$ Deterministic Evaluation:** Every motion primitive compiles into a pure, continuous function $f(t) \to \text{Value}$ evaluated in constant time, ensuring frame-rate independence and glitch-free rendering under CPU load.
* **Declarative Tree Integration:** Composable timing pipelines attach directly to standard GPUI elements via reactive property mappers and fluent combinators, keeping motion co-located with your component layout.

### Key Features

* **Direct Element Binding:** Animate standard GPUI layout properties, custom canvas properties, and reactive variables directly.
* **Analytical Physics:** Exact, closed-form solutions for harmonic springs, piecewise parabolic gravity, and kinetic drag—zero Euler integration drift.
* **Frame-Rate Independent:** Animations evaluate continuously at any refresh rate (60 Hz, 120 Hz, or variable) and support instant random-access timeline scrubbing.
* **Fluent Chaining:** Easily compose complex sequences combining keyframe `.tween()` transitions and physics algorithms.
* **Zero Canvas Lock-In:** Works seamlessly across standard GPUI layout elements (`Div`) and custom canvas draw passes.

## Installation

Add `gpui_animotion` and `gpui` to your `Cargo.toml`:

```toml
[dependencies]
gpui = { package = "gpui-unofficial", version = "1.16" }
gpui_animotion = "0.4"
```

## Quickstart

Clone the repository and run the included example:

```bash
git clone https://github.com/astrimid/gpui_animotion.git
cd gpui_animotion

# Run the multi-primitive kinetic sandbox
cargo run --example kinetic_marble_playground

# Run the synchronized easing & Bezier racetrack
cargo run --example easing_bezier_showcase

# Run the multi-ball gravity simulation
cargo run --example many_shiny_balls
```

## Usage Examples

### 1. Spring & Tween Inline Animation (`.animotion`)

For standard UI components, use `.animotion()` to declaratively map transitions directly onto standard `Div` attributes:

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
                        .tween_eased(0.0, 0.6, Ease::OutCubic),
                    |el, x| el.left(px(x)),
                ),
                // Concurrently pulse scale/opacity
                prop(
                    tween(0.8, 1.1, 0.4)
                        .ease(Ease::Custom(0.68, -0.55, 0.27, 1.55))
                        .tween_eased(0.8, 0.6, Ease::InOutQuad),
                    |el, s| el.opacity(s),
                ),
            )),
        )
}
```

### 2. Multi-Body Physics & Custom Canvas (`.animotion_clip`)

For complex scenes (like multi-body physics simulations), use `.animotion_clip()` to declare tracked properties, chain analytical trajectories, and render via canvas:

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
            // 1. Lively underdamped spring chained into a return bounce
            BallProps {
                x: c.prop(100.0)
                    .tween_eased(160.0, 1.2, Ease::InOutQuad)
                    .tween_eased(100.0, 1.2, Ease::InOutQuad)
                    .clone(),
                y: c.prop(40.0)
                    .spring(floor_y, SpringParams::from_damping_ratio(220.0, 0.22))
                    .spring(40.0, SpringParams::from_damping_ratio(180.0, 0.60))
                    .clone(),
                color: c.prop(hsla(0.75, 0.8, 0.50, 1.0)),
            },
            // 2. Kinetic friction flick chained into an elastic bumper spring
            BallProps {
                x: c.prop(120.0)
                    .flick(FlickParams {
                        initial_velocity: 2800.0,
                        friction: 3.8,
                        threshold: 0.5,
                        duration: None,
                    })
                    .spring(120.0, SpringParams::from_damping_ratio(190.0, 0.45))
                    .clone(),
                y: c.prop(180.0),
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
                            let origin = point(
                                bounds.origin.x + px(ball.x.get()),
                                bounds.origin.y + px(ball.y.get()),
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

### 1. Keyframing, Tweens & Easing (`tween`, `tween_eased`, `Ease`)

Tweens linearly or non-linearly interpolate property values across durations. Interpolation is implemented natively for `f32`, `Hsla`, `Rgba`, and GPUI geometry primitives:

* **Sequential Chaining:** Chain multiple calls together to build multi-step paths (`tween(0.0, 100.0, 0.5).tween(50.0, 0.3)`).
* **Fluent Easing Modifiers:** Apply non-linear transfer curves using `.ease(Ease)` or `.tween_eased(target, secs, ease)`.
* **Standard Presets:** `Linear`, `InQuad`, `OutQuad`, `InOutQuad`, `InCubic`, `OutCubic`, `InOutCubic`, `InExpo`, `OutExpo`, `InOutExpo`, `InBack`, `OutBack`, `InOutBack`, `OutBounce`, `InBounce`, `InOutBounce`.
* **Parametric 4-Point Cubic Bezier:** Solve exact CSS-style curves using `Ease::Custom(x1, y1, x2, y2)` with Newton-Raphson root finding:

```rust
// Custom overshoot and anticipation curve
prop.tween_eased(300.0, 0.8, Ease::Custom(0.68, -0.60, 0.32, 1.60));
```

### 2. Analytical Spring Physics (`spring`)

Spring tracks solve the differential equations of a damped harmonic oscillator ($m\ddot{x} + c\dot{x} + kx = 0$) in closed form, providing smooth motion without numerical integration or frame-rate dependency:

* **`SpringParams`:** Configure mass ($m$), stiffness ($k$), damping ($c$), initial velocity ($v_0$), and settling threshold ($\epsilon$).
* **Damping Ratio Helper (`SpringParams::from_damping_ratio(stiffness, zeta)`):**
* $\zeta < 1.0$: **Underdamped** (elastic bounce with decaying oscillations).
* $\zeta = 1.0$: **Critically Damped** (fastest settlement without overshoot).
* $\zeta > 1.0$: **Overdamped** (friction-dominated smooth settle).

* **Analytical Duration Solving:** The natural resting duration is calculated analytically using logarithmic decay envelopes, or can be bounded by an explicit duration cap.

```rust
// Chain an elastic underdamped bounce into a tight settle
prop.spring(300.0, SpringParams::from_damping_ratio(200.0, 0.35))
    .spring(0.0, SpringParams::from_damping_ratio(180.0, 0.70));
```

### 3. Kinetic Friction Decay (`flick`, `FlickParams`)

Flick tracks model Newtonian drag deceleration ($\dot{v} = -cv$) with closed-form position and velocity evaluation:

* **Gesture Integration:** Pipe finger or cursor release velocity directly into `initial_velocity`.
* **Automatic Duration:** Computes the exact millisecond velocity drops below the resting threshold ($T_{\text{settle}} = \frac{\ln(\vert{}v_0\vert{} / \epsilon)}{c}$).
* **Timed Override:** Provide an optional duration to have the engine calculate the matching friction coefficient automatically.

```rust
prop.flick(FlickParams {
    initial_velocity: swipe_velocity,
    friction: 4.2,
    threshold: 0.5,
    duration: None,
});
```

### 4. Piecewise Parabolic Gravity (`gravity`, `GravityParams`)

* **`gravity(...)`:** Solves ballistic flight arcs and floor collisions analytically using exact kinematic equations ($y(t) = y_0 + v_0 t + \frac{1}{2} g t^2$) and restitution coefficients.
* Computes multi-bounce trajectories in $\mathcal{O}(1)$ time without iterative Euler stepping.
* Eliminates the need to hand-craft bounce parabola keyframes.

### 4. Property Handles (`Prop<T>`)

`Prop<T>` manages animated state across frames:

* **Thread-Safe Sampling:** Backed by thread-safe synchronization primitives (`Arc<Mutex<Track<T>>>`), allowing properties to be sampled with minimal overhead during GPUI layout and paint passes.
* **Continuous State Inspection:** Access position and instantaneous velocity derivatives ($x(t), \dot{x}(t)$) for momentum preservation across interruptions.
* **Fluent Builder:** Attach `.tween()`, `.tween_eased()`, `.spring()`, `.flick()`, and `.gravity()` calls directly to the property handle before mounting into the view tree.

### 6. Combinators (`all` and `seq`)

Orchestrate how multiple properties or elements execute:

* **`all(...)`**: Executes wrapped property tracks in parallel concurrently.
* **`seq(...)`**: Executes wrapped animation tracks sequentially in series *(coming soon)*.

### 7. Continuous Looping

Animations loop seamlessly when a sequence returns to its initial value. Chaining a physics track with an inverse spring or return `.tween()` guarantees visual continuity across continuous playback cycles.

## License

Dual-licensed under either of:

* Apache License, Version 2.0 ([LICENSE-APACHE](http://www.apache.org/licenses/LICENSE-2.0))
* MIT license ([LICENSE-MIT](http://opensource.org/licenses/MIT))
