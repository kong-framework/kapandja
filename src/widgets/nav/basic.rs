use super::selectors::{ACTIVE, LOGO, NAVLINK};
use super::NavItem;
use crate::widgets::Widget;
use csscolorparser::Color;
use maud::html;

pub struct NavBasic {
    pub left_navs: Vec<NavItem>,
    pub right_navs: Vec<NavItem>,
    pub logo: String,
    pub background_color: Color,
    pub link_color: Color,
    pub active_link_color: Color,
}

impl Widget for NavBasic {
    fn html(&self) -> String {
        html!(nav .navbar {
            section .navbar-section{
                @for nav in &self.left_navs {
                    @if nav.active {
                        a .btn.btn-link.(NAVLINK) href=(nav.url) .(ACTIVE){
                            (nav.name)
                        }
                    } @else {
                        a .btn.btn-link.(NAVLINK) href=(nav.url){
                            (nav.name)
                        }
                    }
                }
            }
            section .navbar-center{
                img #(LOGO) src=(self.logo){}
            }
            section .navbar-section{
                @for nav in &self.right_navs {
                    @if nav.active {
                        a .btn.btn-link.(NAVLINK) href=(nav.url){
                            (nav.name)
                        }
                    } @else {
                        a .btn.btn-link.(NAVLINK) href=(nav.url){
                            (nav.name)
                        }
                    }
                }
            }
        })
        .into_string()
    }

    fn css(&self) -> String {
        let background_color = self.background_color.to_rgb_string();
        let link_color = self.link_color.to_rgb_string();
        let active_link_color = self.active_link_color.to_rgb_string();

        format!(
            r#"
nav {{
  background-color: {background_color};
  border: thin solid rgba(0,0,0,0.05);
}}

#LOGO{{
  width: 120px;
}}

.{NAVLINK}{{
  color: {link_color} !important;
  font-family: "Univers-Regular";
}}

.{ACTIVE} {{
  color: {active_link_color} !important;
}}"#
        )
    }
}
