// kala/src/constant.rs
use crate::Composite;
use crate::Format;
use crate::Color;

impl Color {
    // --- A ---
    pub const ALICE_BLUE: Self = Self { inner: Format::RGB { r: 240, g: 248, b: 255 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const ANTIQUE_WHITE: Self = Self { inner: Format::RGB { r: 250, g: 235, b: 215 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const AQUA: Self = Self { inner: Format::RGB { r: 0, g: 255, b: 255 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const AQUAMARINE: Self = Self { inner: Format::RGB { r: 127, g: 255, b: 212 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const AZURE: Self = Self { inner: Format::RGB { r: 240, g: 255, b: 255 }, outer: Composite::AcesAp0, alpha: 1.0 };

    // --- B ---
    pub const BEIGE: Self = Self { inner: Format::RGB { r: 245, g: 245, b: 220 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const BISQUE: Self = Self { inner: Format::RGB { r: 255, g: 228, b: 196 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const BLACK: Self = Self { inner: Format::RGB { r: 0, g: 0, b: 0 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const BLANCHED_ALMOND: Self = Self { inner: Format::RGB { r: 255, g: 235, b: 205 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const BLUE: Self = Self { inner: Format::RGB { r: 0, g: 0, b: 255 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const BLUE_VIOLET: Self = Self { inner: Format::RGB { r: 138, g: 43, b: 226 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const BROWN: Self = Self { inner: Format::RGB { r: 165, g: 42, b: 42 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const BURLYWOOD: Self = Self { inner: Format::RGB { r: 222, g: 184, b: 135 }, outer: Composite::AcesAp0, alpha: 1.0 };

    // --- C ---
    pub const CADET_BLUE: Self = Self { inner: Format::RGB { r: 95, g: 158, b: 160 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const CHARTREUSE: Self = Self { inner: Format::RGB { r: 127, g: 255, b: 0 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const CHOCOLATE: Self = Self { inner: Format::RGB { r: 210, g: 105, b: 30 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const CORAL: Self = Self { inner: Format::RGB { r: 255, g: 127, b: 80 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const CORNFLOWER_BLUE: Self = Self { inner: Format::RGB { r: 100, g: 149, b: 237 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const CORNSILK: Self = Self { inner: Format::RGB { r: 255, g: 248, b: 220 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const CRIMSON: Self = Self { inner: Format::RGB { r: 220, g: 20, b: 60 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const CYAN: Self = Self { inner: Format::RGB { r: 0, g: 255, b: 255 }, outer: Composite::AcesAp0, alpha: 1.0 };

    // --- D ---
    pub const DARK_BLUE: Self = Self { inner: Format::RGB { r: 0, g: 0, b: 139 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const DARK_CYAN: Self = Self { inner: Format::RGB { r: 0, g: 139, b: 139 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const DARK_GOLDENROD: Self = Self { inner: Format::RGB { r: 184, g: 134, b: 11 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const DARK_GRAY: Self = Self { inner: Format::RGB { r: 169, g: 169, b: 169 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const DARK_GREEN: Self = Self { inner: Format::RGB { r: 0, g: 100, b: 0 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const DARK_KHAKI: Self = Self { inner: Format::RGB { r: 189, g: 183, b: 107 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const DARK_MAGENTA: Self = Self { inner: Format::RGB { r: 139, g: 0, b: 139 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const DARK_OLIVE_GREEN: Self = Self { inner: Format::RGB { r: 85, g: 107, b: 47 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const DARK_ORANGE: Self = Self { inner: Format::RGB { r: 255, g: 140, b: 0 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const DARK_ORCHID: Self = Self { inner: Format::RGB { r: 153, g: 50, b: 204 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const DARK_RED: Self = Self { inner: Format::RGB { r: 139, g: 0, b: 0 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const DARK_SALMON: Self = Self { inner: Format::RGB { r: 233, g: 150, b: 122 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const DARK_SEA_GREEN: Self = Self { inner: Format::RGB { r: 143, g: 188, b: 139 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const DARK_SLATE_BLUE: Self = Self { inner: Format::RGB { r: 72, g: 61, b: 139 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const DARK_SLATE_GRAY: Self = Self { inner: Format::RGB { r: 47, g: 79, b: 79 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const DARK_TURQUOISE: Self = Self { inner: Format::RGB { r: 0, g: 206, b: 209 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const DARK_VIOLET: Self = Self { inner: Format::RGB { r: 148, g: 0, b: 211 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const DEEP_PINK: Self = Self { inner: Format::RGB { r: 255, g: 20, b: 147 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const DEEP_SKY_BLUE: Self = Self { inner: Format::RGB { r: 0, g: 191, b: 255 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const DIM_GRAY: Self = Self { inner: Format::RGB { r: 105, g: 105, b: 105 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const DODGER_BLUE: Self = Self { inner: Format::RGB { r: 30, g: 144, b: 255 }, outer: Composite::AcesAp0, alpha: 1.0 };

    // --- F ---
    pub const FIREBRICK: Self = Self { inner: Format::RGB { r: 178, g: 34, b: 34 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const FLORAL_WHITE: Self = Self { inner: Format::RGB { r: 255, g: 250, b: 240 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const FOREST_GREEN: Self = Self { inner: Format::RGB { r: 34, g: 139, b: 34 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const FUCHSIA: Self = Self { inner: Format::RGB { r: 255, g: 0, b: 255 }, outer: Composite::AcesAp0, alpha: 1.0 };

    // --- G ---
    pub const GAINSBORO: Self = Self { inner: Format::RGB { r: 220, g: 220, b: 220 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const GHOST_WHITE: Self = Self { inner: Format::RGB { r: 248, g: 248, b: 255 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const GOLD: Self = Self { inner: Format::RGB { r: 255, g: 215, b: 0 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const GOLDENROD: Self = Self { inner: Format::RGB { r: 218, g: 165, b: 32 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const GRAY: Self = Self { inner: Format::RGB { r: 128, g: 128, b: 128 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const GREEN: Self = Self { inner: Format::RGB { r: 0, g: 128, b: 0 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const GREEN_YELLOW: Self = Self { inner: Format::RGB { r: 173, g: 255, b: 47 }, outer: Composite::AcesAp0, alpha: 1.0 };

    // --- H ---
    pub const HONEYDEW: Self = Self { inner: Format::RGB { r: 240, g: 255, b: 240 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const HOT_PINK: Self = Self { inner: Format::RGB { r: 255, g: 105, b: 180 }, outer: Composite::AcesAp0, alpha: 1.0 };

    // --- I ---
    pub const INDIAN_RED: Self = Self { inner: Format::RGB { r: 205, g: 92, b: 92 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const INDIGO: Self = Self { inner: Format::RGB { r: 75, g: 0, b: 130 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const IVORY: Self = Self { inner: Format::RGB { r: 255, g: 255, b: 240 }, outer: Composite::AcesAp0, alpha: 1.0 };

    // --- K ---
    pub const KHAKI: Self = Self { inner: Format::RGB { r: 240, g: 230, b: 140 }, outer: Composite::AcesAp0, alpha: 1.0 };

    // --- L ---
    pub const LAVENDER: Self = Self { inner: Format::RGB { r: 230, g: 230, b: 250 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const LAVENDER_BLUSH: Self = Self { inner: Format::RGB { r: 255, g: 240, b: 245 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const LAWN_GREEN: Self = Self { inner: Format::RGB { r: 124, g: 252, b: 0 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const LEMON_CHIFFON: Self = Self { inner: Format::RGB { r: 255, g: 250, b: 205 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const LIGHT_BLUE: Self = Self { inner: Format::RGB { r: 173, g: 216, b: 230 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const LIGHT_CORAL: Self = Self { inner: Format::RGB { r: 240, g: 128, b: 128 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const LIGHT_CYAN: Self = Self { inner: Format::RGB { r: 224, g: 255, b: 255 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const LIGHT_GOLDENROD_YELLOW: Self = Self { inner: Format::RGB { r: 250, g: 250, b: 210 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const LIGHT_GRAY: Self = Self { inner: Format::RGB { r: 211, g: 211, b: 211 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const LIGHT_GREEN: Self = Self { inner: Format::RGB { r: 144, g: 238, b: 144 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const LIGHT_PINK: Self = Self { inner: Format::RGB { r: 255, g: 182, b: 193 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const LIGHT_SALMON: Self = Self { inner: Format::RGB { r: 255, g: 160, b: 122 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const LIGHT_SEA_GREEN: Self = Self { inner: Format::RGB { r: 32, g: 178, b: 170 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const LIGHT_SKY_BLUE: Self = Self { inner: Format::RGB { r: 135, g: 206, b: 250 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const LIGHT_SLATE_GRAY: Self = Self { inner: Format::RGB { r: 119, g: 136, b: 153 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const LIGHT_STEEL_BLUE: Self = Self { inner: Format::RGB { r: 176, g: 196, b: 222 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const LIGHT_YELLOW: Self = Self { inner: Format::RGB { r: 255, g: 255, b: 224 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const LIME: Self = Self { inner: Format::RGB { r: 0, g: 255, b: 0 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const LIME_GREEN: Self = Self { inner: Format::RGB { r: 50, g: 205, b: 50 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const LINEN: Self = Self { inner: Format::RGB { r: 250, g: 240, b: 230 }, outer: Composite::AcesAp0, alpha: 1.0 };

    // --- M ---
    pub const MAGENTA: Self = Self { inner: Format::RGB { r: 255, g: 0, b: 255 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const MAROON: Self = Self { inner: Format::RGB { r: 128, g: 0, b: 0 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const MEDIUM_AQUAMARINE: Self = Self { inner: Format::RGB { r: 102, g: 205, b: 170 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const MEDIUM_BLUE: Self = Self { inner: Format::RGB { r: 0, g: 0, b: 205 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const MEDIUM_ORCHID: Self = Self { inner: Format::RGB { r: 186, g: 85, b: 211 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const MEDIUM_PURPLE: Self = Self { inner: Format::RGB { r: 147, g: 112, b: 219 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const MEDIUM_SEA_GREEN: Self = Self { inner: Format::RGB { r: 60, g: 179, b: 113 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const MEDIUM_SLATE_BLUE: Self = Self { inner: Format::RGB { r: 123, g: 104, b: 238 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const MEDIUM_SPRING_GREEN: Self = Self { inner: Format::RGB { r: 0, g: 250, b: 154 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const MEDIUM_TURQUOISE: Self = Self { inner: Format::RGB { r: 72, g: 209, b: 204 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const MEDIUM_VIOLET_RED: Self = Self { inner: Format::RGB { r: 199, g: 21, b: 133 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const MIDNIGHT_BLUE: Self = Self { inner: Format::RGB { r: 25, g: 25, b: 112 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const MINT_CREAM: Self = Self { inner: Format::RGB { r: 245, g: 255, b: 250 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const MISTY_ROSE: Self = Self { inner: Format::RGB { r: 255, g: 228, b: 225 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const MOCCASIN: Self = Self { inner: Format::RGB { r: 255, g: 228, b: 181 }, outer: Composite::AcesAp0, alpha: 1.0 };

    // --- N ---
    pub const NAVY: Self = Self { inner: Format::RGB { r: 0, g: 0, b: 128 }, outer: Composite::AcesAp0, alpha: 1.0 };

    // --- O ---
    pub const OLD_LACE: Self = Self { inner: Format::RGB { r: 253, g: 245, b: 230 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const OLIVE: Self = Self { inner: Format::RGB { r: 128, g: 128, b: 0 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const OLIVE_DRAB: Self = Self { inner: Format::RGB { r: 107, g: 142, b: 35 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const ORANGE: Self = Self { inner: Format::RGB { r: 255, g: 165, b: 0 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const ORANGE_RED: Self = Self { inner: Format::RGB { r: 255, g: 69, b: 0 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const ORCHID: Self = Self { inner: Format::RGB { r: 218, g: 112, b: 214 }, outer: Composite::AcesAp0, alpha: 1.0 };

    // --- P ---
    pub const PALE_GOLDENROD: Self = Self { inner: Format::RGB { r: 238, g: 232, b: 170 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const PALE_GREEN: Self = Self { inner: Format::RGB { r: 152, g: 251, b: 152 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const PALE_TURQUOISE: Self = Self { inner: Format::RGB { r: 175, g: 238, b: 238 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const PALE_VIOLET_RED: Self = Self { inner: Format::RGB { r: 219, g: 112, b: 147 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const PAPAYA_WHIP: Self = Self { inner: Format::RGB { r: 255, g: 239, b: 213 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const PEACH_PUFF: Self = Self { inner: Format::RGB { r: 255, g: 218, b: 185 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const PERU: Self = Self { inner: Format::RGB { r: 205, g: 133, b: 63 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const PINK: Self = Self { inner: Format::RGB { r: 255, g: 192, b: 203 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const PLUM: Self = Self { inner: Format::RGB { r: 221, g: 160, b: 221 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const POWDER_BLUE: Self = Self { inner: Format::RGB { r: 176, g: 224, b: 230 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const PURPLE: Self = Self { inner: Format::RGB { r: 128, g: 0, b: 128 }, outer: Composite::AcesAp0, alpha: 1.0 };

    // --- R ---
    pub const REBECCA_PURPLE: Self = Self { inner: Format::RGB { r: 102, g: 51, b: 153 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const RED: Self = Self { inner: Format::RGB { r: 255, g: 0, b: 0 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const ROSY_BROWN: Self = Self { inner: Format::RGB { r: 188, g: 143, b: 143 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const ROYAL_BLUE: Self = Self { inner: Format::RGB { r: 65, g: 105, b: 225 }, outer: Composite::AcesAp0, alpha: 1.0 };

    // --- S ---
    pub const SADDLE_BROWN: Self = Self { inner: Format::RGB { r: 139, g: 69, b: 19 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const SALMON: Self = Self { inner: Format::RGB { r: 250, g: 128, b: 114 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const SANDY_BROWN: Self = Self { inner: Format::RGB { r: 244, g: 164, b: 96 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const SEA_GREEN: Self = Self { inner: Format::RGB { r: 46, g: 139, b: 87 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const SEASHELL: Self = Self { inner: Format::RGB { r: 255, g: 245, b: 238 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const SIENNA: Self = Self { inner: Format::RGB { r: 160, g: 82, b: 45 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const SILVER: Self = Self { inner: Format::RGB { r: 192, g: 192, b: 192 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const SKY_BLUE: Self = Self { inner: Format::RGB { r: 135, g: 206, b: 235 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const SLATE_BLUE: Self = Self { inner: Format::RGB { r: 106, g: 90, b: 205 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const SLATE_GRAY: Self = Self { inner: Format::RGB { r: 112, g: 128, b: 144 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const SNOW: Self = Self { inner: Format::RGB { r: 255, g: 250, b: 250 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const SPRING_GREEN: Self = Self { inner: Format::RGB { r: 0, g: 255, b: 127 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const STEEL_BLUE: Self = Self { inner: Format::RGB { r: 70, g: 130, b: 180 }, outer: Composite::AcesAp0, alpha: 1.0 };

    // --- T ---
    pub const TAN: Self = Self { inner: Format::RGB { r: 210, g: 180, b: 140 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const TEAL: Self = Self { inner: Format::RGB { r: 0, g: 128, b: 128 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const THISTLE: Self = Self { inner: Format::RGB { r: 216, g: 191, b: 216 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const TOMATO: Self = Self { inner: Format::RGB { r: 255, g: 99, b: 71 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const TURQUOISE: Self = Self { inner: Format::RGB { r: 64, g: 224, b: 208 }, outer: Composite::AcesAp0, alpha: 1.0 };

    // --- V ---
    pub const VIOLET: Self = Self { inner: Format::RGB { r: 238, g: 130, b: 238 }, outer: Composite::AcesAp0, alpha: 1.0 };

    // --- W ---
    pub const WHEAT: Self = Self { inner: Format::RGB { r: 245, g: 222, b: 179 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const WHITE: Self = Self { inner: Format::RGB { r: 255, g: 255, b: 255 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const WHITE_SMOKE: Self = Self { inner: Format::RGB { r: 245, g: 245, b: 245 }, outer: Composite::AcesAp0, alpha: 1.0 };

    // --- Y ---
    pub const YELLOW: Self = Self { inner: Format::RGB { r: 255, g: 255, b: 0 }, outer: Composite::AcesAp0, alpha: 1.0 };
    pub const YELLOW_GREEN: Self = Self { inner: Format::RGB { r: 154, g: 205, b: 50 }, outer: Composite::AcesAp0, alpha: 1.0 };
}