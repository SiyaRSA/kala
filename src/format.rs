// src/Format.rs
use std::hash::Hasher;
use std::hash::Hash;
use set26::Seq;

/// # KALA
/// ## Color Format
/// 
/// Defines the representation used to store and describe a color.
///
/// Each variant represents a distinct color representation or color
/// model, ranging from common formats such as RGB and RGBA to
/// perceptual and device-oriented models such as LAB, LCH, XYZ,
/// and CMYK.
///
/// Format is used as the inner representation of [Color].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Format<const N: usize> {
    /// ## KALA
    /// ### RGBA Color Format
    /// 8-bit unsinged ranging from `0` to `255`.
    RGB { r: u8, g: u8, b: u8 },
    /// ## KALA
    /// ### RGBA Color Format
    /// 8-bit unsinged ranging from `0` to `255`.
    RGBA { r: u8, g: u8, b: u8, a: u8 },
    /// ## KALA
    /// ### HEX Color Format
    /// A 32-bit hexadecimal color representation `(0xRRGGBB)`.
    HEX (u32),
    /// ## KALA
    /// ### HSL Color Format
    HSL { h: f32, s: f32, l: f32 },
    /// ## KALA
    /// ### HSLA Color Format
    HSLA { h: f32, s: f32, l: f32, a: f32 },
    /// ## KALA
    /// ### HSV Color Format
    HSV { h: f32, s: f32, v: f32 },
    /// ## KALA
    /// ### CMYK Color Format
    CMYK { c: f32, m: f32, y: f32, k: f32, },
    /// ## KALA
    /// ### LAB Color Format
    LAB { l: f32, a: f32, b: f32, },
    /// ## KALA
    /// ### LCH Color Format
    LCH { l: f32, c: f32, h: f32, },
    /// ## KALA
    /// ### XYZ Color Format
    XYZ { x: f32, y: f32, z: f32, },
    /// ## KALA
    /// ### YCbCr Color Format
    YCbCr { y: f32, cb: f32, cr: f32, },
    /// ## KALA
    /// ### DATA Color Format
    /// A variable-length color representation using `64-bit`
    /// floating-point values for experimental and scientific
    /// color data.
    DATA(Seq<f64, N>),

    /// ## KALA
    /// ### DATA2 Color Format
    /// A variable-length color representation using 128-bit
    /// floating-point values for high-precision experimental
    /// and scientific color data.
    ///
    /// > ⚠️ **Nightly Requirement**: This variant uses the unstable 
    /// `f128` primitive type, which **requires a nightly Rust compiler** 
    /// and the `#![feature(f128)]` feature gate enabled at the crate root.
    ///
    #[cfg(feature = "experimental")]
    DATA2(Seq<f128, N>),
}

impl<const N: usize> Default for Format<N> {
    fn default() -> Self {
        Format::RGB { 
            r: 0, 
            g: 0, 
            b: 0 
        }
    }
}

impl<const N: usize> Eq for Format<N> {}

impl<const N: usize> Hash for Format<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Format::RGB { r, g, b } => {
                r.hash(state);
                g.hash(state);
                b.hash(state);
            }
            Format::RGBA { r, g, b, a } => {
                r.hash(state);
                g.hash(state);
                b.hash(state);
                a.hash(state);
            }
            Format::HEX(val) => {
                val.hash(state);
            }
            Format::HSL { h, s, l } => {
                h.to_bits().hash(state);
                s.to_bits().hash(state);
                l.to_bits().hash(state);
            }
            Format::HSLA { h, s, l, a } => {
                h.to_bits().hash(state);
                s.to_bits().hash(state);
                l.to_bits().hash(state);
                a.to_bits().hash(state);
            }
            Format::HSV { h, s, v } => {
                h.to_bits().hash(state);
                s.to_bits().hash(state);
                v.to_bits().hash(state);
            }
            Format::CMYK { c, m, y, k } => {
                c.to_bits().hash(state);
                m.to_bits().hash(state);
                y.to_bits().hash(state);
                k.to_bits().hash(state);
            }
            Format::LAB { l, a, b } => {
                l.to_bits().hash(state);
                a.to_bits().hash(state);
                b.to_bits().hash(state);
            }
            Format::LCH { l, c, h } => {
                l.to_bits().hash(state);
                c.to_bits().hash(state);
                h.to_bits().hash(state);
            }
            Format::XYZ { x, y, z } => {
                x.to_bits().hash(state);
                y.to_bits().hash(state);
                z.to_bits().hash(state);
            }
            Format::YCbCr { y, cb, cr } => {
                y.to_bits().hash(state);
                cb.to_bits().hash(state);
                cr.to_bits().hash(state);
            }
            Format::DATA(values) => {
                values.len().hash(state);

                for value in values {
                    value.to_bits().hash(state);
                }
            }
            #[cfg(feature = "experimental")]
            Format::DATA2(values) => {
                values.len().hash(state);

                for value in values {
                    value.to_bits().hash(state);
                }
            }
        }
    }
}

impl<const N: usize> Format<N> {
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        match self {
            Format::RGB { r, g, b } => (*r, *g, *b),
            Format::RGBA { r, g, b, .. } => (*r, *g, *b), // Ignores alpha
            Format::HEX(hex) => {
                let r = ((hex >> 16) & 0xFF) as u8;
                let g = ((hex >> 8) & 0xFF) as u8;
                let b = (hex & 0xFF) as u8;
                (r, g, b)
            }
            // Direct mapping for XYZ where x->r, y->g, z->b
            // Assuming values are normalized 0.0-1.0 or can be cast directly
            Format::XYZ { x, y, z } => {
                let r = (x * 255.0).clamp(0.0, 255.0) as u8;
                let g = (y * 255.0).clamp(0.0, 255.0) as u8;
                let b = (z * 255.0).clamp(0.0, 255.0) as u8;
                (r, g, b)
            }
            // Turns the f64 elements of DATA into u8 (taking the first 3 elements if available)
            Format::DATA(seq) => {
                let r = seq.get(0).copied().unwrap_or(0.0) as u8;
                let g = seq.get(1).copied().unwrap_or(0.0) as u8;
                let b = seq.get(2).copied().unwrap_or(0.0) as u8;
                (r, g, b)
            }
            // Turns the f128 elements of DATA2 into u8
            #[cfg(feature = "experimental")]
            Format::DATA2(seq) => {
                // Cast f128 to u8 via satisfying the primitive cast
                let r = seq.get(0).copied().map(|v| v as u8).unwrap_or(0);
                let g = seq.get(1).copied().map(|v| v as u8).unwrap_or(0);
                let b = seq.get(2).copied().map(|v| v as u8).unwrap_or(0);
                (r, g, b)
            }
            _ => (0,0,0)
        }
    }
}