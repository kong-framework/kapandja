use super::selectors::{HERO, HERO_HEADING};
use crate::widgets::Widget;
use csscolorparser::Color;
use maud::html;

pub struct HeroBasic3 {
    pub image: String,
    pub heading: String,
    pub description: String,
    pub text_color: Color,
    pub border_color: Color,
    pub logo: String,
    pub button_text: String,
    pub button_link: String,
}

impl Widget for HeroBasic3 {
    fn html(&self) -> String {
        html!(section #(HERO) {
            div {img #HERO_LOGO src=(self.logo){}}
            h1 #(HERO_HEADING) {(self.heading)}
            p {(self.description)}
            a href=(self.button_link) .btn.btn-lg {(self.button_text)}
        })
        .into_string()
    }

    fn css(&self) -> String {
        let img = &self.image;
        let text_color = &self.text_color.to_rgb_string();
        let border_color = &self.border_color.to_rgb_string();

        format!(
            r#"
#{HERO} {{
  background-image: url("{img}");
  height: 100vh;
  background-size: 100%;
  background-position: center center;
  font-family: "Univers-Regular";
  background-repeat: no-repeat;
  padding: 4em;
  padding-top: 0;
  text-align: right;
  border: thick solid {border_color};
}}

#{HERO_HEADING}, #{HERO} p {{
  color: {text_color} !important;
}}

#{HERO_HEADING}{{
  font-size: 4em;
  text-shadow: 1px 1px 1px #000000;
}}

#{HERO} p{{
  font-size: 2em;
  text-shadow: 1px 1px 1px #000000;
}}

#{HERO} #HERO_LOGO{{
  width: 150px;
}}

#{HERO} div{{
  text-align: left;
}}

#{HERO} .btn{{
  background-color: {border_color};
  border-color: {border_color};
  color: {text_color};
}}

@media only screen and (max-width: 1183px){{

  #{HERO_HEADING} {{
    font-size: 1.1em;
  }}

  #{HERO} p{{
    font-size: .8em;
  }}

  #{HERO} {{
    padding: 2em;
  }}
}}

@media only screen and (max-width: 1599px){{
  #{HERO} {{
    height: 80vh;
  }}
}}

@media only screen and (max-width: 1276px){{
  #{HERO} {{
    height: 60vh;
  }}

  #{HERO} #HERO_LOGO{{
    width: 100px;
  }}

  #{HERO_HEADING} {{
    font-size: 1.4em;
  }}

  #{HERO} p{{
    font-size: .8em;
  }}

}}

@media only screen and (max-width: 953px){{
  #{HERO} {{
    height: 40vh;
  }}
}}

@media only screen and (max-width: 638px){{
  #{HERO} {{
    height: 30vh;
  }}
}}

@media only screen and (max-width: 638px){{
  #{HERO} {{
    height: 20vh;
  }}

  #{HERO_HEADING} {{
    display: none;
  }}

  #{HERO} p{{
    display: none;
  }}

  #{HERO} .btn{{
    display: none;
  }}

  #{HERO} #HERO_LOGO{{
    width: 70px;
  }}

}}

"#
        )
    }
}
