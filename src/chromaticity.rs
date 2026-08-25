// src/chromaticity
#![allow(unused)]
/// CIE 1931 2° xy chromaticity coordinates for RGB colour spaces.
///
/// Coordinates are:
/// - red   = (x, y)
/// - green = (x, y)
/// - blue  = (x, y)
/// - white = (x, y)
///
/// `linear_*`, encoded variants, and spaces sharing the same primaries
/// intentionally have the same chromaticities.
///
/// Source/reference set:
/// - Colour Science `RGB_COLOURSPACES`
/// - ICC Three Component Color Encoding Registry
///
/// Note:
/// Chromaticity coordinates describe the primaries and white point.
/// They do NOT describe the transfer function/OETF/EOTF.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Chromaticity {
pub x: f64,
pub y: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RgbChromaticities {
    pub red: Chromaticity,
    pub green: Chromaticity,
    pub blue: Chromaticity,
    pub white: Chromaticity,
}

impl Chromaticity {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl RgbChromaticities {
    pub const fn new(
        red:   Chromaticity,
        green: Chromaticity,
        blue:  Chromaticity,
        white: Chromaticity,
    ) -> Self {
        Self {
            red,
            green,
            blue,
            white,
        }
    }
}

const D50: Chromaticity = Chromaticity::new(0.3457, 0.3585);
const D55: Chromaticity = Chromaticity::new(0.3324, 0.3474);
const D60: Chromaticity = Chromaticity::new(0.32168, 0.33767);
const D65: Chromaticity = Chromaticity::new(0.3127, 0.3290);
const E:   Chromaticity = Chromaticity::new(1.0 / 3.0, 1.0 / 3.0);
const DCI: Chromaticity = Chromaticity::new(0.3140, 0.3510);

//
// Common display / video spaces
//

pub const SRGB: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.640, 0.330),
Chromaticity::new(0.300, 0.600),
Chromaticity::new(0.150, 0.060),
D65,
);

pub const REC_709: RgbChromaticities = SRGB;

pub const LINEAR_REC_709: RgbChromaticities = SRGB;

pub const REC_2020: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.708, 0.292),
Chromaticity::new(0.170, 0.797),
Chromaticity::new(0.131, 0.046),
D65,
);

pub const LINEAR_REC_2020: RgbChromaticities = REC_2020;

pub const REC_601_525: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.630, 0.340),
Chromaticity::new(0.310, 0.595),
Chromaticity::new(0.155, 0.070),
D65,
);

pub const REC_601_625: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.640, 0.330),
Chromaticity::new(0.290, 0.600),
Chromaticity::new(0.150, 0.060),
D65,
);

pub const SMPTE_C: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.630, 0.340),
Chromaticity::new(0.310, 0.595),
Chromaticity::new(0.155, 0.070),
Chromaticity::new(0.3127, 0.3290),
);

pub const SMPTE_240M: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.630, 0.340),
Chromaticity::new(0.310, 0.595),
Chromaticity::new(0.155, 0.070),
Chromaticity::new(0.3127, 0.3290),
);

pub const EBU_TECH_3213_E: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.640, 0.330),
Chromaticity::new(0.290, 0.600),
Chromaticity::new(0.150, 0.060),
D65,
);

pub const PAL_SECAM: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.640, 0.330),
Chromaticity::new(0.290, 0.600),
Chromaticity::new(0.150, 0.060),
D65,
);

//
// P3 family
//

pub const DCI_P3: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.680, 0.320),
Chromaticity::new(0.265, 0.690),
Chromaticity::new(0.150, 0.060),
DCI,
);

pub const P3_D65: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.680, 0.320),
Chromaticity::new(0.265, 0.690),
Chromaticity::new(0.150, 0.060),
D65,
);

pub const DISPLAY_P3: RgbChromaticities = P3_D65;

pub const LINEAR_P3_D65: RgbChromaticities = P3_D65;

pub const DCI_P3_P: RgbChromaticities = DCI_P3;

//
// Adobe / photographic spaces
//

pub const ADOBE_RGB_1998: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.640, 0.330),
Chromaticity::new(0.210, 0.710),
Chromaticity::new(0.150, 0.060),
D65,
);

pub const LINEAR_ADOBE_RGB: RgbChromaticities = ADOBE_RGB_1998;

pub const ADOBE_WIDE_GAMUT_RGB: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.7347, 0.2653),
Chromaticity::new(0.1152, 0.8264),
Chromaticity::new(0.1566, 0.0177),
D50,
);

pub const PROPHOTO_RGB: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.7347, 0.2653),
Chromaticity::new(0.1596, 0.8404),
Chromaticity::new(0.0366, 0.0001),
D50,
);

pub const ROMM_RGB: RgbChromaticities = PROPHOTO_RGB;

pub const RIMM_RGB: RgbChromaticities = PROPHOTO_RGB;

pub const ERIMM_RGB: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.7347, 0.2653),
Chromaticity::new(0.1596, 0.8404),
Chromaticity::new(0.0366, 0.0001),
D50,
);

pub const APPLE_RGB: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.625, 0.340),
Chromaticity::new(0.280, 0.595),
Chromaticity::new(0.155, 0.070),
D65,
);

pub const COLORMATCH_RGB: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.630, 0.340),
Chromaticity::new(0.295, 0.605),
Chromaticity::new(0.150, 0.075),
D50,
);

pub const BEST_RGB: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.7347, 0.2653),
Chromaticity::new(0.2150, 0.7750),
Chromaticity::new(0.1300, 0.0350),
D50,
);

pub const BETA_RGB: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.6888, 0.3112),
Chromaticity::new(0.1986, 0.7551),
Chromaticity::new(0.1265, 0.0352),
D50,
);

pub const EKTA_SPACE_PS5: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.695, 0.305),
Chromaticity::new(0.260, 0.700),
Chromaticity::new(0.110, 0.005),
D50,
);

pub const DON_RGB_4: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.696, 0.300),
Chromaticity::new(0.215, 0.765),
Chromaticity::new(0.130, 0.035),
D50,
);

//
// CIE spaces
//

pub const CIE_RGB: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.7347, 0.2653),
Chromaticity::new(0.2738, 0.7174),
Chromaticity::new(0.1666, 0.0089),
E,
);

pub const CIE_XYZ_D65: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(1.0, 0.0),
Chromaticity::new(0.0, 1.0),
Chromaticity::new(0.0, 0.0),
D65,
);

//
// ACES
//

pub const ACES_AP0: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.73470, 0.26530),
Chromaticity::new(0.00000, 1.00000),
Chromaticity::new(0.00010, -0.07700),
D60,
);

pub const ACES_2065_1: RgbChromaticities = ACES_AP0;

pub const ACES_AP1: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.71300, 0.29300),
Chromaticity::new(0.16500, 0.83000),
Chromaticity::new(0.12800, 0.04400),
D60,
);

pub const ACESCG: RgbChromaticities = ACES_AP1;

pub const ACES_CC: RgbChromaticities = ACES_AP1;
pub const ACES_CCT: RgbChromaticities = ACES_AP1;
pub const ACES_PROXY: RgbChromaticities = ACES_AP1;

//
// Camera / cinema wide gamuts
//

pub const ARRI_WIDE_GAMUT_3: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.6840, 0.3130),
Chromaticity::new(0.2210, 0.8480),
Chromaticity::new(0.0861, -0.1020),
D65,
);

pub const ARRI_WIDE_GAMUT_4: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.7347, 0.2653),
Chromaticity::new(0.1424, 0.8576),
Chromaticity::new(0.0991, -0.0308),
D65,
);

pub const BLACKMAGIC_WIDE_GAMUT: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.7177215, 0.3171181),
Chromaticity::new(0.2280410, 0.8615690),
Chromaticity::new(0.1005841, -0.0820452),
D65,
);

pub const DAVINCI_WIDE_GAMUT: RgbChromaticities = BLACKMAGIC_WIDE_GAMUT;

pub const RED_WIDE_GAMUT_RGB: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.780308, 0.304253),
Chromaticity::new(0.121595, 1.000000),
Chromaticity::new(0.095612, -0.084589),
D65,
);

pub const F_GAMUT: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.7347, 0.2653),
Chromaticity::new(0.1424, 0.8576),
Chromaticity::new(0.0991, -0.0308),
D60,
);

pub const F_GAMUT_C: RgbChromaticities = F_GAMUT;

pub const FILMLIGHT_E_GAMUT: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.8000, 0.3130),
Chromaticity::new(0.1000, 0.9000),
Chromaticity::new(0.0100, -0.0800),
D65,
);

pub const FILMLIGHT_E_GAMUT_2: RgbChromaticities = FILMLIGHT_E_GAMUT;

pub const V_GAMUT: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.730, 0.280),
Chromaticity::new(0.165, 0.840),
Chromaticity::new(0.100, -0.030),
D65,
);

//
// Sony
//

pub const S_GAMUT: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.730, 0.280),
Chromaticity::new(0.140, 0.855),
Chromaticity::new(0.100, -0.050),
D65,
);

pub const S_GAMUT3: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.730, 0.280),
Chromaticity::new(0.140, 0.855),
Chromaticity::new(0.100, -0.050),
D65,
);

pub const S_GAMUT3_CINE: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.7660, 0.2750),
Chromaticity::new(0.2250, 0.8000),
Chromaticity::new(0.0890, -0.0870),
D65,
);

pub const VENICE_S_GAMUT3: RgbChromaticities = S_GAMUT3;

pub const VENICE_S_GAMUT3_CINE: RgbChromaticities = S_GAMUT3_CINE;

//
// Other camera gamuts
//

pub const DJI_D_GAMUT: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.7100, 0.3100),
Chromaticity::new(0.1700, 0.7900),
Chromaticity::new(0.0800, -0.0300),
D65,
);

pub const CINEMA_GAMUT: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.7347, 0.2653),
Chromaticity::new(0.1900, 0.8000),
Chromaticity::new(0.1000, -0.0500),
D65,
);

pub const N_GAMUT: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.7347, 0.2653),
Chromaticity::new(0.1400, 0.8550),
Chromaticity::new(0.1000, -0.0500),
D65,
);

//
// Legacy / miscellaneous RGB spaces
//

pub const NTSC_1953: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.670, 0.330),
Chromaticity::new(0.210, 0.710),
Chromaticity::new(0.140, 0.080),
D65,
);

pub const NTSC_1987: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.630, 0.340),
Chromaticity::new(0.310, 0.595),
Chromaticity::new(0.155, 0.070),
D65,
);

pub const SHARP_RGB: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.630, 0.340),
Chromaticity::new(0.310, 0.595),
Chromaticity::new(0.155, 0.070),
D65,
);

pub const RUSSELL_RGB: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.700, 0.300),
Chromaticity::new(0.210, 0.710),
Chromaticity::new(0.140, 0.080),
D65,
);

pub const MAX_RGB: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.7347, 0.2653),
Chromaticity::new(0.1400, 0.8500),
Chromaticity::new(0.1000, -0.0500),
D65,
);

pub const PLASA_ANSI_E1_54: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.680, 0.320),
Chromaticity::new(0.265, 0.690),
Chromaticity::new(0.150, 0.060),
D65,
);

pub const PROTUNE_NATIVE: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.640, 0.330),
Chromaticity::new(0.300, 0.600),
Chromaticity::new(0.150, 0.060),
D65,
);

pub const XTREME_RGB: RgbChromaticities = RgbChromaticities::new(
Chromaticity::new(0.7347, 0.2653),
Chromaticity::new(0.1400, 0.8500),
Chromaticity::new(0.1000, -0.0500),
D65,
);

//
// H.273 / generic encodings
//

pub const ITU_T_H273_22_UNSPECIFIED: RgbChromaticities = SRGB;

pub const ITU_T_H273_GENERIC_FILM: RgbChromaticities = SRGB;
