// src/operations.rs
/*use crate::color::Color;

impl Color {
    /// Adjusts the lightness by a given amount (e.g., 0.1 for +10%).
    pub fn lighten(&self, amount: f32) -> Self {
        let (h, s, l, a) = self.to_hsla();
        let new_l = (l + amount).clamp(0.0, 1.0);
        Self::from_hsla(h, s, new_l, a)
    }

    /// Darkens the color by a given amount (e.g., 0.1 for -10%).
    pub fn darken(&self, amount: f32) -> Self {
        let (h, s, l, a) = self.to_hsla();
        let new_l = (l - amount).clamp(0.0, 1.0);
        Self::from_hsla(h, s, new_l, a)
    }

    /// Increases color saturation by a given amount.
    pub fn saturate(&self, amount: f32) -> Self {
        let (h, s, l, a) = self.to_hsla();
        let new_s = (s + amount).clamp(0.0, 1.0);
        Self::from_hsla(h, new_s, l, a)
    }

    /// Decreases color saturation by a given amount.
    pub fn desaturate(&self, amount: f32) -> Self {
        let (h, s, l, a) = self.to_hsla();
        let new_s = (s - amount).clamp(0.0, 1.0);
        Self::from_hsla(h, new_s, l, a)
    }

    /// Increases opacity (fade in) by a given amount.
    pub fn fade_in(&self, amount: f32) -> Self {
        let (r, g, b, a) = self.to_rgba_f32();
        let new_a = (a + amount).clamp(0.0, 1.0);
        Self::rgba(r, g, b, new_a)
    }

    /// Decreases opacity (fade out / transparency) by a given amount.
    pub fn fade_out(&self, amount: f32) -> Self {
        let (r, g, b, a) = self.to_rgba_f32();
        let new_a = (a - amount).clamp(0.0, 1.0);
        Self::rgba(r, g, b, new_a)
    }

    /// Inverts the color (bitwise or channel inversion for RGB).
    pub fn invert(&self) -> Self {
        let (r, g, b, a) = self.to_rgba_f32();
        Self::rgba(255 - r, 255 - g, 255 - b, a)
    }

    /// Linear interpolation (lerp) blending between this color and another by a ratio (0.0 to 1.0).
    pub fn blend(&self, other: &Color, ratio: f32) -> Self {
        let ratio = ratio.clamp(0.0, 1.0);
        let (r1, g1, b1, a1) = self.to_rgba_f32();
        let (r2, g2, b2, a2) = other.to_rgba_f32();

        let r = ((1.0 - ratio) * (r1 as f32) + ratio * (r2 as f32)).round() as u8;
        let g = ((1.0 - ratio) * (g1 as f32) + ratio * (g2 as f32)).round() as u8;
        let b = ((1.0 - ratio) * (b1 as f32) + ratio * (b2 as f32)).round() as u8;
        let a = (1.0 - ratio) * a1 + ratio * a2;

        Self::rgba(r, g, b, a)
    }
}*/