// src/util/matrix.rs
#![allow(unused)]
/// # KALA
/// A 2×3 matrix of 32-bit floating-point values.
///
/// The matrix is represented as three rows, each containing two `f32`
/// values:
///
/// ```text
/// [
///     [m₀₀, m₀₁],
///     [m₁₀, m₁₁],
///     [m₂₀, m₂₁],
/// ]
/// ```
///
/// Commonly used for 3-channel color transformations, such as converting
/// between RGB, XYZ, and other color representations.
pub type Matrix2 = [[f32; 2]; 3];

/// # KALA
/// A 3×3 matrix of 32-bit floating-point values.
///
/// The matrix is represented as three rows, each containing three `f32`
/// values:
///
/// ```text
/// [
///     [m₀₀, m₀₁, m₀₂],
///     [m₁₀, m₁₁, m₁₂],
///     [m₂₀, m₂₁, m₂₂],
/// ]
/// ```
///
/// Commonly used for 3-channel color transformations, such as converting
/// between RGB, XYZ, and other color representations.
pub type Matrix3 = [[f32; 3]; 3];

/// # KALA
///
/// A 2-dimensional point of 32-bit floating-point values.
///
/// The point is represented as two `f32` values:
///
/// ```text
/// [
///     x,
///     y,
/// ]
/// ```
///
/// Commonly used for CIE xy chromaticity coordinates, such as
/// RGB primaries and white points.
pub type Point2 = [f32; 2];