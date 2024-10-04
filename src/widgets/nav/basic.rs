use crate::widgets::Widget;
use maud::html;

pub struct NavBasic {}

impl Widget for NavBasic {
    fn html(&self) -> String {
        html!(nav {}).into_string()
    }
}
