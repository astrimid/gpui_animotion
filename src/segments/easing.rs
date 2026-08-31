/// Easing curve profiles for non-linear timing transitions.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum Ease {
    #[default]
    Linear,

    // Quadratic (t^2)
    InQuad,
    OutQuad,
    InOutQuad,

    // Cubic (t^3)
    InCubic,
    OutCubic,
    InOutCubic,

    // Quartic (t^4)
    InQuart,
    OutQuart,
    InOutQuart,

    // Exponential (2^(10(t-1)))
    InExpo,
    OutExpo,
    InOutExpo,

    // Back (Anticipation / Overshoot)
    InBack,
    OutBack,
    InOutBack,

    // Bounce (Ballistic multi-bounce decay)
    InBounce,
    OutBounce,
    InOutBounce,

    // CSS Web Defaults
    CssEase,       // cubic-bezier(0.25, 0.1, 0.25, 1.0)
    CssEaseIn,     // cubic-bezier(0.42, 0.0, 1.0, 1.0)
    CssEaseOut,    // cubic-bezier(0.0, 0.0, 0.58, 1.0)
    CssEaseInOut,  // cubic-bezier(0.42, 0.0, 0.58, 1.0)

    /// Custom 4-point parametric Cubic Bezier (x1, y1, x2, y2).
    /// Control points (0, 0) and (1, 1) are implicit.
    Custom(f32, f32, f32, f32),
}

impl Ease {
    /// Evaluates the transfer function: transforms normalized time t ∈ [0, 1] into eased progress τ ∈ [0, 1].
    pub fn sample(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Ease::Linear => t,

            // Quadratic
            Ease::InQuad => t * t,
            Ease::OutQuad => t * (2.0 - t),
            Ease::InOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }

            // Cubic
            Ease::InCubic => t * t * t,
            Ease::OutCubic => {
                let p = t - 1.0;
                p * p * p + 1.0
            }
            Ease::InOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    let p = 2.0 * t - 2.0;
                    0.5 * p * p * p + 1.0
                }
            }

            // Quartic
            Ease::InQuart => t * t * t * t,
            Ease::OutQuart => {
                let p = t - 1.0;
                1.0 - p * p * p * p
            }
            Ease::InOutQuart => {
                if t < 0.5 {
                    8.0 * t * t * t * t
                } else {
                    let p = t - 1.0;
                    1.0 - 8.0 * p * p * p * p
                }
            }

            // Exponential
            Ease::InExpo => {
                if t == 0.0 {
                    0.0
                } else {
                    (2.0f32).powf(10.0 * (t - 1.0))
                }
            }
            Ease::OutExpo => {
                if t == 1.0 {
                    1.0
                } else {
                    1.0 - (2.0f32).powf(-10.0 * t)
                }
            }
            Ease::InOutExpo => {
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else if t < 0.5 {
                    0.5 * (2.0f32).powf(20.0 * t - 10.0)
                } else {
                    1.0 - 0.5 * (2.0f32).powf(-20.0 * t + 10.0)
                }
            }

            // Back (Overshoot)
            Ease::InBack => {
                let s = 1.70158;
                t * t * ((s + 1.0) * t - s)
            }
            Ease::OutBack => {
                let s = 1.70158;
                let p = t - 1.0;
                p * p * ((s + 1.0) * p + s) + 1.0
            }
            Ease::InOutBack => {
                let s = 1.70158 * 1.525;
                let p = t * 2.0;
                if p < 1.0 {
                    0.5 * (p * p * ((s + 1.0) * p - s))
                } else {
                    let p2 = p - 2.0;
                    0.5 * (p2 * p2 * ((s + 1.0) * p2 + s) + 2.0)
                }
            }

            // Bounce
            Ease::OutBounce => Self::bounce_out(t),
            Ease::InBounce => 1.0 - Self::bounce_out(1.0 - t),
            Ease::InOutBounce => {
                if t < 0.5 {
                    0.5 * (1.0 - Self::bounce_out(1.0 - t * 2.0))
                } else {
                    0.5 * Self::bounce_out(t * 2.0 - 1.0) + 0.5
                }
            }

            // Presets via Cubic Bezier
            Ease::CssEase => CubicBezier::new(0.25, 0.1, 0.25, 1.0).sample(t),
            Ease::CssEaseIn => CubicBezier::new(0.42, 0.0, 1.0, 1.0).sample(t),
            Ease::CssEaseOut => CubicBezier::new(0.0, 0.0, 0.58, 1.0).sample(t),
            Ease::CssEaseInOut => CubicBezier::new(0.42, 0.0, 0.58, 1.0).sample(t),

            // Custom Curve
            Ease::Custom(x1, y1, x2, y2) => CubicBezier::new(*x1, *y1, *x2, *y2).sample(t),
        }
    }

    /// Evaluates the first derivative dτ/dt at progress t for velocity calculation.
    pub fn derivative(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Ease::Linear => 1.0,
            Ease::InQuad => 2.0 * t,
            Ease::OutQuad => 2.0 - 2.0 * t,
            Ease::InOutQuad => {
                if t < 0.5 {
                    4.0 * t
                } else {
                    4.0 - 4.0 * t
                }
            }
            Ease::InCubic => 3.0 * t * t,
            Ease::OutCubic => {
                let p = t - 1.0;
                3.0 * p * p
            }
            Ease::InOutCubic => {
                if t < 0.5 {
                    12.0 * t * t
                } else {
                    let p = 2.0 * t - 2.0;
                    3.0 * p * p
                }
            }
            // Fallback finite difference for piecewise/transcendental functions
            _ => {
                let h = 0.001;
                let t0 = (t - h).max(0.0);
                let t1 = (t + h).min(1.0);
                (self.sample(t1) - self.sample(t0)) / (t1 - t0)
            }
        }
    }

    fn bounce_out(mut t: f32) -> f32 {
        let n1 = 7.5625;
        let d1 = 2.75;

        if t < 1.0 / d1 {
            n1 * t * t
        } else if t < 2.0 / d1 {
            t -= 1.5 / d1;
            n1 * t * t + 0.75
        } else if t < 2.5 / d1 {
            t -= 2.25 / d1;
            n1 * t * t + 0.9375
        } else {
            t -= 2.625 / d1;
            n1 * t * t + 0.984375
        }
    }
}

/// 4-point Parametric Cubic Bezier Solver.
/// Computes y given x(t) = timeline_progress using Newton-Raphson root finding.
#[derive(Copy, Clone, Debug)]
pub struct CubicBezier {
    ax: f32,
    bx: f32,
    cx: f32,
    ay: f32,
    by: f32,
    cy: f32,
}

impl CubicBezier {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        // Polynomial coefficients for x(t) = ax*t^3 + bx*t^2 + cx*t
        let cx = 3.0 * x1;
        let bx = 3.0 * (x2 - x1) - cx;
        let ax = 1.0 - cx - bx;

        // Polynomial coefficients for y(t) = ay*t^3 + by*t^2 + cy*t
        let cy = 3.0 * y1;
        let by = 3.0 * (y2 - y1) - cy;
        let ay = 1.0 - cy - by;

        Self {
            ax,
            bx,
            cx,
            ay,
            by,
            cy,
        }
    }

    #[inline(always)]
    fn sample_curve_x(&self, t: f32) -> f32 {
        ((self.ax * t + self.bx) * t + self.cx) * t
    }

    #[inline(always)]
    fn sample_curve_y(&self, t: f32) -> f32 {
        ((self.ay * t + self.by) * t + self.cy) * t
    }

    #[inline(always)]
    fn sample_curve_derivative_x(&self, t: f32) -> f32 {
        (3.0 * self.ax * t + 2.0 * self.bx) * t + self.cx
    }

    /// Solves for parameter `t` given `x` using Newton-Raphson iteration with bisection fallback.
    fn solve_curve_t(&self, x: f32) -> f32 {
        let mut t = x;

        // 1. Try Newton-Raphson iteration (Fast convergence for smooth curves)
        for _ in 0..8 {
            let current_x = self.sample_curve_x(t) - x;
            if current_x.abs() < 1e-6 {
                return t;
            }
            let dx = self.sample_curve_derivative_x(t);
            if dx.abs() < 1e-6 {
                break; // Slope too flat; fallback to bisection
            }
            t -= current_x / dx;
        }

        // 2. Fallback: Bisection search within [0.0, 1.0]
        let mut t_low = 0.0;
        let mut t_high = 1.0;
        t = x;

        while t_low < t_high {
            let current_x = self.sample_curve_x(t);
            if (current_x - x).abs() < 1e-6 {
                return t;
            }
            if x > current_x {
                t_low = t;
            } else {
                t_high = t;
            }
            t = (t_high + t_low) * 0.5;
        }

        t
    }

    /// Evaluates the eased output y given input timeline progress x ∈ [0, 1].
    pub fn sample(&self, x: f32) -> f32 {
        if x <= 0.0 {
            return 0.0;
        }
        if x >= 1.0 {
            return 1.0;
        }
        let t = self.solve_curve_t(x);
        self.sample_curve_y(t)
    }
}
