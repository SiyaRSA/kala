// src/map.rs
use crate::chromaticity::RgbChromaticities;
use crate::chromaticity::ADOBE_RGB_1998;
use crate::chromaticity::PROPHOTO_RGB;
use crate::chromaticity::ACES_2065_1;
use crate::chromaticity::CIE_XYZ_D65;
use crate::chromaticity::ACES_AP0;
use crate::chromaticity::REC_2020;
use crate::chromaticity::CIE_RGB;
use crate::chromaticity::ACES_CC;
use crate::chromaticity::REC_709;
use crate::chromaticity::ACESCG;
use crate::chromaticity::DCI_P3;
use crate::chromaticity::P3_D65;
use crate::chromaticity::SRGB;

/// # KALA
/// Short List of RGB Chromaticities
pub const MAP: &[RgbChromaticities] = &[
    ADOBE_RGB_1998,
    PROPHOTO_RGB,
    ACES_2065_1,
    CIE_XYZ_D65,
    ACES_AP0,
    REC_2020,
    CIE_RGB,
    ACES_CC,
    REC_709,
    ACESCG,
    P3_D65,
    DCI_P3,
    SRGB,
];