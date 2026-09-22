use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug)]
struct Sample1D {
    time: Instant,
    value: f32,
}

/// Tracks 1D pointer/scalar movements to calculate exit velocity on release.
#[derive(Clone, Debug)]
pub struct VelocityTracker {
    samples: Vec<Sample1D>,
    window: Duration,
}

impl Default for VelocityTracker {
    fn default() -> Self {
        Self::new(Duration::from_millis(120))
    }
}

impl VelocityTracker {
    pub fn new(window: Duration) -> Self {
        Self {
            samples: Vec::with_capacity(16),
            window,
        }
    }

    /// Records a new scalar sample at the current instant.
    pub fn push(&mut self, value: f32) {
        let now = Instant::now();
        self.samples.push(Sample1D { time: now, value });
        self.prune(now);
    }

    /// Resets the tracker history.
    pub fn reset(&mut self) {
        self.samples.clear();
    }

    fn prune(&mut self, now: Instant) {
        let cutoff = now.checked_sub(self.window).unwrap_or(now);
        self.samples.retain(|s| s.time >= cutoff);
    }

    /// Calculates instantaneous exit velocity in units/second using least-squares linear regression.
    pub fn velocity(&mut self) -> f32 {
        let now = Instant::now();
        self.prune(now);

        if self.samples.len() < 2 {
            return 0.0;
        }

        // If latest sample is stale (>60ms old), the gesture stopped before release
        if let Some(last) = self.samples.last() {
            if now.duration_since(last.time) > Duration::from_millis(60) {
                return 0.0;
            }
        }

        let first_time = self.samples[0].time;
        let mut sum_t = 0.0f32;
        let mut sum_x = 0.0f32;
        let mut sum_tt = 0.0f32;
        let mut sum_tx = 0.0f32;
        let n = self.samples.len() as f32;

        for s in &self.samples {
            let t = s.time.duration_since(first_time).as_secs_f32();
            let x = s.value;
            sum_t += t;
            sum_x += x;
            sum_tt += t * t;
            sum_tx += t * x;
        }

        let denominator = n * sum_tt - sum_t * sum_t;
        if denominator.abs() < 1e-6 {
            return 0.0;
        }

        // Slope of linear regression (dx/dt)
        (n * sum_tx - sum_t * sum_x) / denominator
    }
}

#[derive(Clone, Copy, Debug)]
struct Sample2D {
    time: Instant,
    x: f32,
    y: f32,
}

/// Tracks 2D pointer coordinates to calculate directional exit velocity vectors.
#[derive(Clone, Debug)]
pub struct VelocityTracker2D {
    samples: Vec<Sample2D>,
    window: Duration,
}

impl Default for VelocityTracker2D {
    fn default() -> Self {
        Self::new(Duration::from_millis(120))
    }
}

impl VelocityTracker2D {
    pub fn new(window: Duration) -> Self {
        Self {
            samples: Vec::with_capacity(16),
            window,
        }
    }

    pub fn push(&mut self, x: f32, y: f32) {
        let now = Instant::now();
        self.samples.push(Sample2D { time: now, x, y });
        self.prune(now);
    }

    pub fn reset(&mut self) {
        self.samples.clear();
    }

    fn prune(&mut self, now: Instant) {
        let cutoff = now.checked_sub(self.window).unwrap_or(now);
        self.samples.retain(|s| s.time >= cutoff);
    }

    /// Returns directional exit velocity vector `(vx, vy)` in pixels/second.
    pub fn velocity(&mut self) -> (f32, f32) {
        let now = Instant::now();
        self.prune(now);

        if self.samples.len() < 2 {
            return (0.0, 0.0);
        }

        if let Some(last) = self.samples.last() {
            if now.duration_since(last.time) > Duration::from_millis(60) {
                return (0.0, 0.0);
            }
        }

        let first_time = self.samples[0].time;
        let mut sum_t = 0.0f32;
        let mut sum_x = 0.0f32;
        let mut sum_y = 0.0f32;
        let mut sum_tt = 0.0f32;
        let mut sum_tx = 0.0f32;
        let mut sum_ty = 0.0f32;
        let n = self.samples.len() as f32;

        for s in &self.samples {
            let t = s.time.duration_since(first_time).as_secs_f32();
            sum_t += t;
            sum_x += s.x;
            sum_y += s.y;
            sum_tt += t * t;
            sum_tx += t * s.x;
            sum_ty += t * s.y;
        }

        let denominator = n * sum_tt - sum_t * sum_t;
        if denominator.abs() < 1e-6 {
            return (0.0, 0.0);
        }

        let vx = (n * sum_tx - sum_t * sum_x) / denominator;
        let vy = (n * sum_ty - sum_t * sum_y) / denominator;

        (vx, vy)
    }
}
