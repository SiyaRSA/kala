// src/format.rs
use crate::chromaticity::*;

/// # KALA
/// ## Color Composite
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Composite {
    Srgb,
    Rec709,
    LinearRec709,
    Rec2020,
    LinearRec2020,

    Rec601525,
    Rec601625,
    SmpteC,
    Smpte240M,
    EbuTech3213E,
    PalSecam,

    DciP3,
    P3D65,
    DisplayP3,
    LinearP3D65,

    AdobeRgb1998,
    LinearAdobeRgb,
    AdobeWideGamutRgb,
    ProPhotoRgb,
    RimmRgb,
    RommRgb,

    CieRgb,
    CieXyzD65,

    AcesAp0,
    Aces20651,
    AcesAp1,
    AcesCg,
    AcesCc,
    AcesCct,
    AcesProxy,

    ArriWideGamut3,
    ArriWideGamut4,
    BlackmagicWideGamut,
    DavinciWideGamut,
    RedWideGamutRgb,
    FGamut,
    FGamutC,
    FilmLightEGamut,
    FilmLightEGamut2,
    VGamut,

    SGamut,
    SGamut3,
    SGamut3Cine,
    VeniceSGamut3,
    VeniceSGamut3Cine,

    DjiDGamut,
    CinemaGamut,
    NGamut,

    Ntsc1953,
    Ntsc1987,
    SharpRgb,
    RussellRgb,
    MaxRgb,
    PlasaAnsiE154,
    ProtuneNative,
    XtremeRgb,

    ItuTH27322Unspecified,
    ItuTH273GenericFilm,
}

impl Composite {
    pub const fn chromaticity(self) -> RgbChromaticities {
        match self {
            Self::Srgb => SRGB,
            Self::Rec709 => REC_709,
            Self::LinearRec709 => LINEAR_REC_709,

            Self::Rec2020 => REC_2020,
            Self::LinearRec2020 => LINEAR_REC_2020,

            Self::Rec601525 => REC_601_525,
            Self::Rec601625 => REC_601_625,

            Self::SmpteC => SMPTE_C,
            Self::Smpte240M => SMPTE_240M,
            Self::EbuTech3213E => EBU_TECH_3213_E,
            Self::PalSecam => PAL_SECAM,

            Self::DciP3 => DCI_P3,
            Self::P3D65 => P3_D65,
            Self::DisplayP3 => DISPLAY_P3,
            Self::LinearP3D65 => LINEAR_P3_D65,

            Self::AdobeRgb1998 => ADOBE_RGB_1998,
            Self::LinearAdobeRgb => LINEAR_ADOBE_RGB,
            Self::AdobeWideGamutRgb => ADOBE_WIDE_GAMUT_RGB,
            Self::ProPhotoRgb => PROPHOTO_RGB,
            Self::RimmRgb => RIMM_RGB,
            Self::RommRgb => ROMM_RGB,

            Self::CieRgb => CIE_RGB,
            Self::CieXyzD65 => CIE_XYZ_D65,

            Self::AcesAp0 => ACES_AP0,
            Self::Aces20651 => ACES_2065_1,
            Self::AcesAp1 => ACES_AP1,
            Self::AcesCg => ACESCG,
            Self::AcesCc => ACES_CC,
            Self::AcesCct => ACES_CCT,
            Self::AcesProxy => ACES_PROXY,

            Self::ArriWideGamut3 => ARRI_WIDE_GAMUT_3,
            Self::ArriWideGamut4 => ARRI_WIDE_GAMUT_4,
            Self::BlackmagicWideGamut => BLACKMAGIC_WIDE_GAMUT,
            Self::DavinciWideGamut => DAVINCI_WIDE_GAMUT,
            Self::RedWideGamutRgb => RED_WIDE_GAMUT_RGB,
            Self::FGamut => F_GAMUT,
            Self::FGamutC => F_GAMUT_C,
            Self::FilmLightEGamut => FILMLIGHT_E_GAMUT,
            Self::FilmLightEGamut2 => FILMLIGHT_E_GAMUT_2,
            Self::VGamut => V_GAMUT,

            Self::SGamut => S_GAMUT,
            Self::SGamut3 => S_GAMUT3,
            Self::SGamut3Cine => S_GAMUT3_CINE,
            Self::VeniceSGamut3 => VENICE_S_GAMUT3,
            Self::VeniceSGamut3Cine => VENICE_S_GAMUT3_CINE,

            Self::DjiDGamut => DJI_D_GAMUT,
            Self::CinemaGamut => CINEMA_GAMUT,
            Self::NGamut => N_GAMUT,

            Self::Ntsc1953 => NTSC_1953,
            Self::Ntsc1987 => NTSC_1987,
            Self::SharpRgb => SHARP_RGB,
            Self::RussellRgb => RUSSELL_RGB,
            Self::MaxRgb => MAX_RGB,
            Self::PlasaAnsiE154 => PLASA_ANSI_E1_54,
            Self::ProtuneNative => PROTUNE_NATIVE,
            Self::XtremeRgb => XTREME_RGB,

            Self::ItuTH27322Unspecified => ITU_T_H273_22_UNSPECIFIED,
            Self::ItuTH273GenericFilm => ITU_T_H273_GENERIC_FILM,
        }
    }
}


impl Default for Composite {
    fn default() -> Self {
        Self::AcesAp0
    }
}