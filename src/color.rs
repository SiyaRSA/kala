// src/color.rs
use crate::format::Format;
use crate::Composite;
use set26::Seq;

/// # KALA
/// ### [Color] S2FsYQ==
/// easy color representation
#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub(crate) inner: Format<3>,
    pub(crate) outer: Composite,
    pub(crate) alpha: f32,
}

impl Color {
    /// Creates a new `Color` from an RGB (**R**ed, **G**reen, **B**lue) composite.<br>
    /// 8-bit unsigned values ranging from `0` to `255`.
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { 
            inner: Format::RGB { r, g, b } ,
            outer: Composite::default(),
            alpha: 1.0,
        }
    }

    /// Creates a new `Color` from an RGBA composite.<br>
    /// **A**lpha is passed as a float (0.0 to 1.0) and converted to an 8-bit `u8` (0 to 255).
    pub fn rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        let aa = (a.clamp(0.0, 1.0) * 255.0).round() as u8;
        Self { 
            inner: Format::RGBA { r, g, b, a:aa },
            outer: Composite::default(),
            alpha: a,
        }
    }

    /// Creates a new `Color` from a 32-bit **HEX** color representation.
    pub fn hex(val: u32) -> Self {
        // Check if the u32 has 4 bytes (contains alpha) vs 3 bytes (pure RGB)
        // This assumes 32-bit values > 0xFFFFFF carry an alpha channel in the lowest byte (RGBA)
        let (rgb, alpha) = if val > 0xFFFFFF {
            let a = ((val & 0xFF) as f32) / 255.0;
            let rgb = val >> 8;
            (rgb, a)
        } else {
            (val, 1.0) // Default to opaque if no alpha channel is in the hex
        };

        Self { 
            inner: Format::HEX(rgb),
            outer: Composite::default(),
            alpha,
        }
    }

    /// Creates a new `Color` from HSL.<br>
    /// **H**ue: 0 to 360 degrees (accepts i32 for natural input, wraps or clamps).<br>
    /// **S**aturation & **L**ightness: 0 to 100 percentage integers, converted to 0.0 to 1.0 floats.
    pub fn hsl(h: i32, s: u8, l: u8) -> Self {
        let h = (h % 360) as f32;
        let s = s.clamp(0, 100) as f32 / 100.0;
        let l = l.clamp(0, 100) as f32 / 100.0;
        Self { 
            inner: Format::HSL { h, s, l },
            outer: Composite::default(),
            alpha: 1.0,
        }
    }

    /// Creates a new `Color` from HSLA.
    pub fn hsla(h: i32, s: u8, l: u8, a: f32) -> Self {
        let h = (h % 360) as f32;
        let s = s.clamp(0, 100) as f32 / 100.0;
        let l = l.clamp(0, 100) as f32 / 100.0;
        let a = (a.clamp(0.0, 1.0) * 255.0).round();
        Self { 
            inner: Format::HSLA { h, s, l, a },
            outer: Composite::default(),
            alpha: a,
        }
    }

    /// Creates a new `Color` from HSV.<br>
    /// Hue: 0 to 360 degrees. Saturation & Value: 0 to 100 percentages.
    pub fn hsv(h: i32, s: u8, v: u8) -> Self {
        let h = (h % 360) as f32;
        let s = s.clamp(0, 100) as f32 / 100.0;
        let v = v.clamp(0, 100) as f32 / 100.0;
        Self { 
            inner: Format::HSV { h, s, v },
            outer: Composite::default(),
            alpha: 1.0,
        }
    }

    /// Creates a new `Color` from CMYK.<br>
    /// Typically expressed as percentages from `0` to `100` for each channel.
    pub fn cmyk(c: u8, m: u8, y: u8, k: u8) -> Self {
        let c = c.clamp(0, 100) as f32 / 100.0;
        let m = m.clamp(0, 100) as f32 / 100.0;
        let y = y.clamp(0, 100) as f32 / 100.0;
        let k = k.clamp(0, 100) as f32 / 100.0;
        Self { 
            inner: Format::CMYK { c, m, y, k },
            outer: Composite::default(),
            alpha: 1.0,
        }
    }

    /// Creates a new `Color` from a LAB (CIELAB) color composite.<br>
    /// L: 0 to 100, A: -128 to 127, B: -128 to 127.
    pub fn lab(l: i32, a: i32, b: i32) -> Self {
        let l = l.clamp(0, 100) as f32;
        let a = a.clamp(-128, 127) as f32;
        let b = b.clamp(-128, 127) as f32;
        Self { 
            inner: Format::LAB { l, a, b },
            outer: Composite::default(),
            alpha: 1.0,
        }
    }

    /// Creates a new `Color` from an LCH (CIELCh) color composite.<br>
    /// **L**ightness: 0 to 100.<br>
    /// **C**hroma: 0 to 150+ (clamped or passed directly as float).<br>
    /// **H**ue: 0 to 360 degrees (accepts i32, wrapped via modulo).
    pub fn lch(l: i32, c: i32, h: i32) -> Self {
        let l = l.clamp(0, 100) as f32;
        let c = c.clamp(0, 230) as f32; // Practical upper bound for standard displayable chroma
        let h = (h % 360) as f32;
        Self { 
            inner: Format::LCH { l, c, h },
            outer: Composite::default(),
            alpha: 1.0,
        }
    }

    /// Creates a new `Color` from CIE 1931 XYZ.<br>
    /// Often standard reference white scales up to 100.0 for Y, but kept as raw floats.
    pub fn xyz(x: f32, y: f32, z: f32) -> Self {
        Self { 
            inner: Format::XYZ { x, y, z },
            outer: Composite::default(),
            alpha: 1.0,
        }
    }

    /// Creates a new `Color` from a YCbCr digital color composite.<br>
    /// **Y** ranges from 0 to 255 (or 16-235), **Cb/Cr** from -128 to 127.
    pub fn ycbcr(y: i32, cb: i32, cr: i32) -> Self {
        let y = y.clamp(0, 255) as f32;
        let cb = cb.clamp(-128, 127) as f32;
        let cr = cr.clamp(-128, 127) as f32;
        Self { 
            inner: Format::YCbCr { y, cb, cr },
            outer: Composite::default(),
            alpha: 1.0,
        }
    }

    /// Creates an experimental scientific data-driven color wrapper using `64-bit` floats.
    pub fn data<const N: usize>(values: Seq<f64, N>) -> Format<N> {
        Format::DATA(values)
    }

    /// Creates a high-precision experimental scientific data-driven color wrapper using `128-bit` floats.
    /// 
    /// > ⚠️ **Nightly Requirement**: Requires the `#![feature(f128)]` feature gate.
    #[cfg(feature = "experimental")]
    pub fn data2<const N: usize>(values: Seq<f128, N>) -> Format<N> {
        Format::DATA(values)
    }
}