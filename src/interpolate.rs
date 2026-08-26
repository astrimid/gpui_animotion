use gpui::Hsla;

/// Trait for types that can be linearly interpolated.
pub trait Interpolate {
    fn interpolate(&self, target: &Self, t: f32) -> Self;
}

impl Interpolate for f32 {
    fn interpolate(&self, target: &Self, t: f32) -> Self {
        self + (target - self) * t
    }
}

impl Interpolate for Hsla {
    fn interpolate(&self, target: &Self, t: f32) -> Self {
        Hsla {
            h: self.h + (target.h - self.h) * t,
            s: self.s + (target.s - self.s) * t,
            l: self.l + (target.l - self.l) * t,
            a: self.a + (target.a - self.a) * t,
        }
    }
}
