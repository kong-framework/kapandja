#![recursion_limit = "512"]

pub use csscolorparser::parse as color;
pub use kapandja::Kapandja;

mod kapandja;
pub mod widgets;
