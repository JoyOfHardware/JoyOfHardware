use zoon::*;
use crate::cssparser_color::RgbaLegacy;

pub const border_gray : RgbaLegacy = color!("#DDDDDD");
pub const fill_gray   : RgbaLegacy = color!("#f0f0f0");

pub const border_red      : RgbaLegacy = color!("#f8d7da");
pub const fill_red        : RgbaLegacy = color!("#dc3545");

pub const border_blue     : RgbaLegacy = color!("#cce5ff");
pub const fill_blue       : RgbaLegacy = color!("#007bff");

pub const border_green    : RgbaLegacy = color!("#d4edda");
pub const fill_green      : RgbaLegacy = color!("#28a745");

pub const border_orange   : RgbaLegacy = color!("#ffe5d1");
pub const fill_orange     : RgbaLegacy = color!("#fd7e14");

pub const border_sky_blue : RgbaLegacy = color!("#d1ecf1");
pub const fill_sky_blue   : RgbaLegacy = color!("#17a2b8");

pub fn is_mobile(width : u32) -> bool {
    width < 760
}