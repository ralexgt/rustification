use crate::kinds::*;

/// Combines two primary colors in equal amounts to create
/// a secondary color.
pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
    match c1 == c2 {
        true => SecondaryColor::Green,
        false => SecondaryColor::Purple,
    }
}