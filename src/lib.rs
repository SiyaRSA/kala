// src/lib.rs
#![cfg_attr(feature = "experimental", feature(f128))]
mod composite;
mod format;
mod color;
mod map;


pub mod chromaticity;
pub mod white_point;
pub mod operations;
pub mod primaries;
pub mod util {
    mod matrix;
    mod calc;
    pub use set26::seq;
    pub use matrix::Point2;
    pub use matrix::Matrix2;
}

pub use composite::Composite;
pub use format::Format;
pub use color::Color;
pub use map::MAP;