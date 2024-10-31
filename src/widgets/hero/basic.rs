use super::selectors::{HERO, HERO_BTN, HERO_HEADING, HERO_SUB_HEADING};
use crate::widgets::Widget;
use csscolorparser::Color;
use maud::html;

pub struct HeroBasic {
    pub image: String,
    pub heading: String,
    pub subheading: String,
    pub heading_color: Color,
    pub subheading_color: Color,
    pub button_text: String,
    pub button_url: String,
    pub button_border_color: Color,
    pub button_text_color: Color,
    pub button_background_color: Color,
}

impl Widget for HeroBasic {
    fn html(&self) -> String {
        html!(section #(HERO) {
            h3 #(HERO_HEADING) {(self.heading)}
            p #(HERO_SUB_HEADING) .col-8.col-lg-4.col-md-8 {(self.subheading)}
            button #(HERO_BTN) .btn{a href=(self.button_url){(self.button_text)}}
        })
        .into_string()
    }

    fn css(&self) -> String {
        let img = &self.image;
        let heading_color = &self.heading_color.to_rgb_string();
        let subheading_color = &self.subheading_color.to_rgb_string();
        let button_background_color = &self.button_background_color.to_rgb_string();
        let button_border_color = &self.button_border_color.to_rgb_string();
        let button_text_color = &self.button_text_color.to_rgb_string();

        format!(
            r#"
#{HERO} {{
  background-image: url("{img}");
  height: 45vh;
  background-size: 100%;
  background-repeat: no-repeat;
  padding: 4em;
}}

#{HERO_HEADING} {{
  color: {heading_color} !important;
  font-family: "Univers-Regular";
}}

#{HERO_SUB_HEADING}{{
  color: {subheading_color} !important;
  font-family: "Univers-Regular";
  text-shadow: 1px 1px 1px #000;
}}

#{HERO} #{HERO_BTN}{{
 border-color: {button_border_color} !important;
 background-color: {button_background_color} !important;
 font-family: "Univers-Regular";
 border-radius: 0;
}}

#{HERO} #{HERO_BTN} a{{
 color: {button_text_color} !important;
}}

@media only screen and (max-width: 1183px){{
  #{HERO} {{
    height: 30vh; 
  }}

  #{HERO_HEADING} {{
    font-size: 1.1em;
  }}

  #{HERO} {{
    padding: 2em;
  }}
}}

@media only screen and (max-width: 397px){{
  #{HERO} {{
    height: 25vh;
  }}

 #{HERO_BTN} {{
    display: none;
  }}

  #{HERO_SUB_HEADING}{{
    font-size: .8em;
  }}
}}

@media only screen and (max-width: 335px){{
  #{HERO} {{
    height: 20vh;
  }}
}}

@media only screen and (max-width: 274px){{
  #{HERO} {{
    height: 15vh;
  }}

  #{HERO_SUB_HEADING}{{
    display: none
  }}
}}
"#
        )
    }
}
