use super::selectors::{HERO, HERO_HEADING};
use crate::widgets::Widget;
use csscolorparser::Color;
use maud::html;

pub struct HeroBasic2 {
    pub image: String,
    pub heading: String,
    pub heading_color: Color,
}

impl Widget for HeroBasic2 {
    fn html(&self) -> String {
        html!(section #(HERO) {
            h3 #(HERO_HEADING) {(self.heading)}
        })
        .into_string()
    }

    fn css(&self) -> String {
        let img = &self.image;
        let heading_color = &self.heading_color.to_rgb_string();

        format!(
            r#"
#{HERO} {{
  background-image: url("{img}");
  height: 60vh;
  background-size: 100%;
  background-position: center center;
  background-repeat: no-repeat;
  padding: 4em;
  text-align: center;
}}

#{HERO_HEADING} {{
  color: {heading_color} !important;
  font-family: "Univers-Regular";
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
}}
"#
        )
    }
}
