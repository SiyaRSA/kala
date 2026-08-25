// src/util/calc.rs
use crate::chromaticity::RgbChromaticities;
use crate::Color;

/*#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Kala {
    maxtix: [[f64;2];4],
    space: Composite,
}

impl Composite {
    pub fn output(&self) -> Kala {
        let c = self.chromaticity();

        let maxtix = [
            [c.red.x, c.red.y],
            [c.green.x, c.green.y],
            [c.blue.x, c.blue.y],
            [c.white.x, c.white.y],
        ];

        let space = *self;

        Kala {
            maxtix,
            space,
        }
    }
}*/

impl Color {
    pub fn output(&self) -> (
        [u8;3],
        RgbChromaticities,
        f32,
    ) {
        let f = self.inner.to_rgb();
        let c = self.outer.chromaticity();

        (
            [
                f.0,
                f.1,
                f.2,
            ],
            c,
            self.alpha,
        )
    }
}