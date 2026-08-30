use crate::segment::AnimationSegment;
use std::time::Duration;

#[derive(Clone, Debug)]
enum SpringRegime {
    Underdamped { decay: f32, omega_d: f32, a: f32, b: f32 },
    CriticallyDamped { omega_n: f32, a: f32, b: f32 },
    Overdamped { decay: f32, omega_d: f32, a: f32, b: f32 },
}

#[derive(Clone, Debug)]
pub struct SpringSegment {
    pub start: f32,
    pub target: f32,
    pub duration: Duration,
    regime: SpringRegime,
}

impl SpringSegment {
    pub fn new(start: f32, target: f32, params: crate::SpringParams) -> Self {
        let x0 = start - target;
        let v0 = params.initial_velocity;
        let mass = params.mass.max(0.001);
        let stiffness = params.stiffness.max(0.001);
        let damping = params.damping.max(0.0);
        let threshold = params.threshold.max(0.0001);

        let omega_n = (stiffness / mass).sqrt();
        let zeta = damping / (2.0 * (stiffness * mass).sqrt());

        let (regime, duration_secs) = if zeta < 0.9999 {
            let omega_d = omega_n * (1.0 - zeta * zeta).sqrt();
            let a = x0;
            let b = (v0 + zeta * omega_n * x0) / omega_d;
            let amplitude = (a * a + b * b).sqrt();
            let dur = if let Some(d) = params.duration {
                d
            } else if amplitude <= threshold {
                0.0
            } else {
                (amplitude / threshold).ln() / (zeta * omega_n)
            };
            (SpringRegime::Underdamped { decay: zeta * omega_n, omega_d, a, b }, dur)
        } else if zeta > 1.0001 {
            let omega_d = omega_n * (zeta * zeta - 1.0).sqrt();
            let a = x0;
            let b = (v0 + zeta * omega_n * x0) / omega_d;
            let lambda_slow = omega_n * (zeta - (zeta * zeta - 1.0).sqrt());
            let dur = if let Some(d) = params.duration {
                d
            } else if x0.abs() <= threshold {
                0.0
            } else {
                (x0.abs() / threshold).ln() / lambda_slow
            };
            (SpringRegime::Overdamped { decay: zeta * omega_n, omega_d, a, b }, dur)
        } else {
            let a = x0;
            let b = v0 + omega_n * x0;
            let r_crit = a.abs() + b.abs() / (omega_n * std::f32::consts::E);
            let dur = if let Some(d) = params.duration {
                d
            } else if r_crit <= threshold {
                0.0
            } else {
                ((r_crit / threshold).ln() + 1.0) / omega_n
            };
            (SpringRegime::CriticallyDamped { omega_n, a, b }, dur)
        };

        Self {
            start,
            target,
            duration: Duration::from_secs_f32(duration_secs.max(0.0)),
            regime,
        }
    }
}

impl AnimationSegment<f32> for SpringSegment {
    fn duration(&self) -> Duration {
        self.duration
    }

    fn evaluate(&self, t: Duration) -> f32 {
        let t_sec = t.as_secs_f32();
        if t_sec <= 0.0 { return self.start; }
        if t >= self.duration { return self.target; }

        let displacement = match &self.regime {
            SpringRegime::Underdamped { decay, omega_d, a, b } => {
                let env = (-decay * t_sec).exp();
                env * (a * (omega_d * t_sec).cos() + b * (omega_d * t_sec).sin())
            }
            SpringRegime::CriticallyDamped { omega_n, a, b } => {
                let env = (-omega_n * t_sec).exp();
                env * (a + b * t_sec)
            }
            SpringRegime::Overdamped { decay, omega_d, a, b } => {
                let env = (-decay * t_sec).exp();
                env * (a * (omega_d * t_sec).cosh() + b * (omega_d * t_sec).sinh())
            }
        };

        self.target + displacement
    }

    fn velocity(&self, t: Duration) -> f32 {
        let t_sec = t.as_secs_f32();
        if t_sec <= 0.0 {
            return match &self.regime {
                SpringRegime::Underdamped { decay, omega_d, a, b } => b * omega_d - decay * a,
                SpringRegime::CriticallyDamped { omega_n, a, b } => b - omega_n * a,
                SpringRegime::Overdamped { decay, omega_d, a, b } => b * omega_d - decay * a,
            };
        }
        if t >= self.duration { return 0.0; }

        match &self.regime {
            SpringRegime::Underdamped { decay, omega_d, a, b } => {
                let env = (-decay * t_sec).exp();
                let cos_t = (omega_d * t_sec).cos();
                let sin_t = (omega_d * t_sec).sin();
                let x = env * (a * cos_t + b * sin_t);
                let dx = env * (-a * omega_d * sin_t + b * omega_d * cos_t);
                -decay * x + dx
            }
            SpringRegime::CriticallyDamped { omega_n, a, b } => {
                let env = (-omega_n * t_sec).exp();
                env * (b - omega_n * (a + b * t_sec))
            }
            SpringRegime::Overdamped { decay, omega_d, a, b } => {
                let env = (-decay * t_sec).exp();
                let cosh_t = (omega_d * t_sec).cosh();
                let sinh_t = (omega_d * t_sec).sinh();
                let x = env * (a * cosh_t + b * sinh_t);
                let dx = env * (a * omega_d * sinh_t + b * omega_d * cosh_t);
                -decay * x + dx
            }
        }
    }

    fn end_value(&self) -> f32 {
        self.target
    }
}
