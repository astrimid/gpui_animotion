# gpui_animotion

[![Crates.io](https://img.shields.io/crates/v/gpui_animotion.svg)](https://crates.io/crates/gpui_animotion)
[![Documentation](https://docs.rs/gpui_animotion/badge.svg)](https://docs.rs/gpui_animotion)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

A declarative, combinator-driven procedural animation engine for [GPUI](https://github.com/zed-industries/zed).

Inspired by the procedural composition of **Motion Canvas** and the timeline orchestration of **GSAP**, `gpui_animotion` brings fine-grained timeline choreography, parametric easing, analytical physical simulations, and momentum-preserving gesture dynamics directly to native Rust UI components.

## Overview & Paradigm

UI animation approaches generally fall into three paradigms, each with distinct tradeoffs:

1. **Implicit Layout Transitions:** Optimized for basic $A \to B$ state changes, but cumbersome when orchestrating multi-element scenes with precise timing dependencies.
2. **Keyframe Timelines:** Highly controllable, but often imperative, verbose, and decoupled from modern reactive layout trees.
3. **Force-Based Game Physics:** Expressive and interactive, but frame-rate dependent, prone to overshoot under CPU jitter, and difficult to synchronize deterministically.

`gpui_animotion` bridges these paradigms for GPUI by pairing **closed-form analytical segment pipelines** with **fluent, combinator-driven element animation**.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                             Track<T> Pipeline                               │
├───────────────────┬─────────────────────────────────┬───────────────────────┤
│ Segment 1:        │ Segment 2:                      │ Segment 3:            │
│ HoldSegment (0.4s)│ ConstrainedSegment (Fit 0.8s)   │ TweenSegment (0.5s)   │
│ [Initial Pause]   │ ├─ Inner: SpringSegment (1.8s)  │ [Eased Return]        │
│                   │ └─ Scale factor: 1.8 / 0.8      │                       │
└────────┬──────────┴────────────────┬────────────────┴───────────┬───────────┘
         │                           │                            │
         ▼                           ▼                            ▼
   evaluate(t)                 evaluate(t)                  evaluate(t)
   velocity(t)                 velocity(t)                  velocity(t)
                                     │
                                     ▼
                     Loop Policy (LoopMode)
       [Once | LoopForever | Count(n) | PingPong | PingPongCount(n)]

```

Instead of maintaining imperative timeline controllers, pre-baking discrete keyframe arrays, or running frame-by-frame numerical integration loops:

* **$\mathcal{O}(1)$ Deterministic Evaluation:** Every motion primitive compiles into a pure, continuous function $f(t) \to \text{Value}$ evaluated in constant time, ensuring frame-rate independence and glitch-free rendering under CPU load.
* **$C^1$ Continuity Interruption:** Interrupt running animations mid-flight without positional snaps or lost momentum; new targets automatically inherit instantaneous velocity vectors ($\dot{x}(t)$).
* **Temporal Normalization:** Untimed physical equations (springs, friction decays) can be scaled or clamped into strict design-system duration budgets (`.fit_to_duration()`, `.clamp_at_duration()`) without losing their characteristic dynamics.
* **Declarative Tree Integration:** Composable timing pipelines attach directly to standard GPUI elements via reactive property mappers and fluent combinators, keeping motion co-located with your component layout.

### Key Features

* **Direct Element Binding:** Animate standard GPUI layout properties, custom canvas properties, and reactive variables directly.
* **Analytical Physics:** Exact, closed-form solutions for harmonic springs, piecewise parabolic gravity, and kinetic drag—zero Euler integration drift.
* **Parametric Easing & Bezier Engine:** Complete suite of standard easing transfer functions alongside an analytical 4-point parametric Cubic Bezier root-finder.
* **Temporal Constraints & Choreography:** Dilate physical trajectories into exact millisecond budgets, inject identity holds/delays, and configure playback cycling policies.
* **Interactive Momentum Continuity:** Hot-swap running animation targets mid-flight (`interrupt_spring`, `interrupt_flick`, `interrupt_tween`) with full velocity preservation.
* **Gesture Velocity Tracking:** Built-in 1D and 2D least-squares regression velocity trackers (`VelocityTracker`, `VelocityTracker2D`) to capture pointer exit momentum for realistic throws and flicks.
* **Frame-Rate Independent:** Animations evaluate continuously at any refresh rate (**60 Hz**, **120 Hz**, or variable) and support instant random-access timeline scrubbing.
* **Fluent Chaining:** Easily compose complex sequences combining keyframe transitions, easing modifiers, and physical trajectories.
* **Zero Canvas Lock-In:** Works seamlessly across standard GPUI layout elements (`Div`) and custom canvas draw passes.


## Installation

Add `gpui_animotion` and `gpui` to your `Cargo.toml`:

```toml
[dependencies]
gpui = { package = "gpui-unofficial", version = "1.16" }
gpui_animotion = "0.4"

```

## Quickstart

Clone the repository and run the included examples:

```bash
git clone https://github.com/astrimid/gpui_animotion.git
cd gpui_animotion

# Run the interactive mouse drag, fling, and spring-snap demo
cargo run --example interactive_throw

# Run the temporal choreography and constraints showcase
cargo run --example choreography_showcase

# Run the multi-primitive kinetic sandbox
cargo run --example kinetic_marble_playground

# Run the synchronized easing & Bezier racetrack
cargo run --example easing_bezier_showcase

# Run the multi-ball gravity simulation
cargo run --example many_shiny_balls

```

## Usage Examples

### 1. Spring, Easing & Tween Inline Animation (`.animotion`)

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
                // X-axis: Analytical spring to 200px, then smooth eased return
                prop(
                    spring(0.0, 200.0, SpringParams::from_damping_ratio(250.0, 0.55))
                        .tween_eased(0.0, 0.6, Ease::OutCubic),
                    |el, x| el.left(px(x)),
                ),
                // Concurrently pulse scale/opacity with custom Bezier overshoot
                prop(
                    tween(0.8, 1.1, 0.4)
                        .ease(Ease::Custom(0.68, -0.55, 0.27, 1.55))
                        .tween_eased(0.8, 0.6, Ease::InOutQuad),
                    |el, s| el.scale(s),
                ),
            )),
        )
}

```

### 2. Choreographed Sequencing with Constraints (`.animotion_clip`)

Compose staggered delays, time-scaled physics, holds, and automatic ping-pong cycles:

```rust
use gpui::*;
use gpui_animotion::*;

fn render_choreographed_card(c: &mut ClipBuilder) -> Prop<f32> {
    c.prop(0.0)
        // 1. Initial 300ms entrance delay
        .delay(0.30)
        // 2. High-oscillation spring dilated to complete in exactly 750ms
        .spring(300.0, SpringParams::from_damping_ratio(140.0, 0.25))
        .fit_to_duration(0.75)
        // 3. Pause at destination for 250ms
        .hold(0.25)
        // 4. Mirror playback automatically back to origin
        .ping_pong()
        .clone()
}

```

### 3. Interactive Gesture Fling & Spring Snap (`interrupt_*`)

Capture live mouse drag movements, compute exit fling velocities via regression, and smoothly reel the element back to its dock:

```rust
use gpui::*;
use gpui_animotion::*;

struct DraggableView {
    pos_x: Prop<f32>,
    pos_y: Prop<f32>,
    tracker: VelocityTracker2D,
}

impl DraggableView {
    fn on_drag(&mut self, current_pos: Point<f32>, cx: &mut Context<Self>) {
        self.tracker.push(current_pos.x, current_pos.y);
        // Direct tracking without lag
        self.pos_x.interrupt_tween(current_pos.x, 0.001, Ease::Linear);
        self.pos_y.interrupt_tween(current_pos.y, 0.001, Ease::Linear);
        cx.notify();
    }

    fn on_release(&mut self, home_x: f32, home_y: f32, cx: &mut Context<Self>) {
        let (vx, vy) = self.tracker.velocity();
        let speed = (vx * vx + vy * vy).sqrt();

        if speed > 100.0 {
            // Ballistic throw: friction glide inherits exit velocity, then springs back
            self.pos_x
                .interrupt_flick(FlickParams {
                    initial_velocity: vx,
                    friction: 3.5,
                    threshold: 1.0,
                    duration: None,
                })
                .spring(home_x, SpringParams::from_damping_ratio(160.0, 0.50));

            self.pos_y
                .interrupt_flick(FlickParams {
                    initial_velocity: vy,
                    friction: 3.5,
                    threshold: 1.0,
                    duration: None,
                })
                .spring(home_y, SpringParams::from_damping_ratio(160.0, 0.50));
        } else {
            // Gentle release: direct spring snap to home
            self.pos_x.interrupt_spring(home_x, SpringParams::from_damping_ratio(180.0, 0.60));
            self.pos_y.interrupt_spring(home_y, SpringParams::from_damping_ratio(180.0, 0.60));
        }
        cx.notify();
    }
}

```

### 4. Multi-Body Physics & Custom Canvas (`.animotion_clip`)

For complex scenes and canvas rendering, declare tracked properties, chain analytical trajectories, and render via canvas:

```rust
use gpui::*;
use gpui_animotion::*;

#[derive(Clone)]
struct BallProps {
    x: Prop<f32>,
    y: Prop<f32>,
    color: Prop<Hsla>,
}

fn render_kinetic_marbles(floor_y: f32) -> impl IntoElement {
    div().animotion_clip(
        ElementId::Name("physics_kinetic_marbles".into()),
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
                                        size: size(px(52.0), px(52.0)),
                                    },
                                    ball.color.get(),
                                )
                                .corner_radii(Corners::all(px(26.0))),
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

Tweens interpolate property values across durations linearly or via transfer functions. Supported natively for `f32`, `Hsla`, `Rgba`, and GPUI geometry primitives:

* **Sequential Chaining:** Chain calls together to build multi-step paths (`tween(0.0, 100.0, 0.5).tween(50.0, 0.3)`).
* **Fluent Easing Modifiers:** Apply non-linear transfer curves using `.ease(Ease)` or `.tween_eased(target, secs, ease)`.
* **Standard Presets:** `Linear`, `InQuad`, `OutQuad`, `InOutQuad`, `InCubic`, `OutCubic`, `InOutCubic`, `InExpo`, `OutExpo`, `InOutExpo`, `InBack`, `OutBack`, `InOutBack`, `OutBounce`, `InBounce`, `InOutBounce`, and CSS presets (`CssEase`, `CssEaseIn`, `CssEaseOut`, `CssEaseInOut`).
* **Parametric 4-Point Cubic Bezier:** Solve exact CSS-style curves using `Ease::Custom(x1, y1, x2, y2)` with Newton-Raphson root finding:

```rust
// Custom overshoot and anticipation curve
prop.tween_eased(300.0, 0.8, Ease::Custom(0.68, -0.60, 0.32, 1.60));

```

### 2. Analytical Spring Physics (`spring`, `SpringParams`)

Spring tracks solve the differential equations of a damped harmonic oscillator ($m\ddot{x} + c\dot{x} + kx = 0$) in closed form:

* **`SpringParams`:** Configure mass ($m$), stiffness ($k$), damping ($c$), initial velocity ($v_0$), and settling threshold ($\epsilon$).
* **Damping Ratio Helper (`SpringParams::from_damping_ratio(stiffness, zeta)`):**
* $\zeta < 1.0$: **Underdamped** (elastic bounce with decaying oscillations).
* $\zeta = 1.0$: **Critically Damped** (fastest settlement without overshoot).
* $\zeta > 1.0$: **Overdamped** (friction-dominated smooth settle).


* **Analytical Duration Solving:** Natural lifespan is calculated analytically using logarithmic decay envelopes.

### 3. Kinetic Friction Decay (`flick`, `FlickParams`)

Flick tracks model Newtonian drag deceleration ($\dot{v} = -cv$) with closed-form position and velocity evaluation:

* **Gesture Integration:** Pipe finger or cursor release velocity directly into `initial_velocity`.
* **Automatic Duration:** Computes the exact timestamp velocity drops below the resting threshold:

$$T_{\text{settle}} = \frac{\ln(\vert{}v_0\vert{} / \epsilon)}{c}$$

* **Timed Override:** Provide an explicit duration to have the engine calculate the matching friction coefficient automatically.

```rust
prop.flick(FlickParams {
    initial_velocity: swipe_velocity,
    friction: 4.2,
    threshold: 0.5,
    duration: None,
});

```

### 4. Piecewise Parabolic Gravity (`gravity`, `GravityParams`)

* **`gravity(...)`:** Solves ballistic flight arcs and floor collisions analytically using kinematic equations ($y(t) = y_0 + v_0 t + \frac{1}{2} g t^2$) and restitution coefficients.
* Computes multi-bounce trajectories in $\mathcal{O}(1)$ time without iterative Euler stepping.

### 5. Temporal Constraints (`fit_to_duration`, `clamp_at_duration`)

Adapt untimed physical segments into strict layout timelines:

* **`.fit_to_duration(secs)`:** Linearly dilates internal time ($t \cdot \frac{T_{\text{natural}}}{T_{\text{target}}}$), ensuring the full physical curve completes within an exact time budget.
* **`.clamp_at_duration(secs)`:** Truncates evaluation at the duration cap and snaps directly to resting equilibrium, eliminating long-tail sub-pixel computations.

### 6. Choreography & Holds (`delay`, `hold`)

* **`.delay(secs)`:** Injects an identity pause at the start or between segments.
* **`.hold(secs)`:** Freezes the preceding segment's end value for a set duration before subsequent actions execute.

### 7. Playback Policies (`LoopMode`)

Configure how tracks advance, cycle, or terminate:

| Method | Mode | Behavior |
| --- | --- | --- |
| `.loop_forever()` | `LoopMode::LoopForever` | Cycles continuously via modulo arithmetic ($t \pmod T$). |
| `.play_once()` | `LoopMode::Once` | Plays through once and rests at the final value ($t = \min(t_{\text{elapsed}}, T)$). |
| `.loop_count(n)` | `LoopMode::Count(n)` | Repeats for $n$ complete cycles, then freezes at the destination. |
| `.ping_pong()` | `LoopMode::PingPong` | Alternates forward and reverse passes continuously without jump cuts. |
| `.ping_pong_count(n)` | `LoopMode::PingPongCount(n)` | Alternates forward and reverse for $n$ half-cycles, then stops. |

### 8. Interactive Dynamics & Interruption Continuity

When users interact with elements already in motion, restarting an animation from scratch resets velocity to zero, causing jarring visual kinks ($C^1$ discontinuities). `Prop<T>` provides continuity-preserving interruption APIs:

* **`prop.interrupt_spring(target, params)`:** Clears pending segments and creates a new spring to `target`, automatically inheriting running velocity ($v_0 = \dot{x}_{\text{current}}$).
* **`prop.interrupt_flick(params)`:** Redirects current motion into an exponential deceleration glide.
* **`prop.interrupt_tween(target, secs, ease)`:** Retargets the property via an eased transition starting from current coordinates.
* **`prop.stop()`:** Freezes the property instantly at current position, resetting velocity to zero.

### 9. Gesture Velocity Tracking (`VelocityTracker`, `VelocityTracker2D`)

Accurately measures release momentum for drag, throw, and swipe interactions:

* **Least-Squares Linear Regression:** Analyzes pointer movements across a sliding temporal window (**100–150 ms**) to eliminate noisy single-frame pointer jitter.
* **Stale Motion Detection:** Detects if the pointer halted before button release and gracefully zeroes exit velocity.

```rust
let mut tracker = VelocityTracker2D::default();

// On pointer move:
tracker.push(x, y);

// On pointer release:
let (vx, vy) = tracker.velocity(); // pixels per second

```

### 10. Reactive Property Handles (`Prop<T>`)

* **Thread-Safe Sampling:** Backed by thread-safe synchronization primitives (`Arc<Mutex<Track<T>>>`), allowing properties to be sampled concurrently during GPUI layout and paint passes.
* **Continuous State Inspection:** Access position (`.get()`) and instantaneous velocity derivatives (`.velocity()`) at any frame.
* **External Handle Attachment:** Attach pre-existing properties into clip pipelines using `builder.attach(&prop)`.

### 11. Parallel Combinators (`all`)

Orchestrate concurrent properties on a single element:

* **`all(...)`:** Bundles tuples of property bindings `(prop1, prop2, ...)` into parallel execution tracks on standard GPUI elements.

## Roadmap

* [x] **Phase 1: Analytical Segment Migration** (Closed-form `AnimationSegment` trait, piecewise parabolic gravity).
* [x] **Phase 2: Kinetic Decay & Easing Suite** (`FlickSegment`, 4-point parametric `CubicBezier`, `Ease` enum, `tween_eased`).
* [x] **Phase 3: Temporal Constraints & Choreography** (`ConstrainedSegment`, `HoldSegment`, `LoopMode`, `.fit_to_duration`, `.ping_pong`).
* [x] **Phase 4: Interactive UI Dynamics & Interruption Continuity** (`Prop::interrupt_*`, velocity inheritance, `VelocityTracker`, `VelocityTracker2D`).
* [ ] **Phase 5: Master Timeline & Headless Motion Canvas Mode** (Multi-track orchestration, timeline scrubbers, deterministic headless video exporter).

## License

Dual-licensed under either of:

* Apache License, Version 2.0 ([LICENSE-APACHE](http://www.apache.org/licenses/LICENSE-2.0))
* MIT license ([LICENSE-MIT](http://opensource.org/licenses/MIT))
