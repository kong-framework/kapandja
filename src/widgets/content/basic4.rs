use super::selectors::CONTENT_HEADING;
use crate::widgets::Widget;
use csscolorparser::Color;
use maud::html;

pub struct ContentBasic4 {
    pub heading: String,
    pub heading_color: Color,
    pub background_image: String,
}

impl Widget for ContentBasic4 {
    fn html(&self) -> String {
        html!(section .basic4-content-section {
            p #(CONTENT_HEADING) {(self.heading)}
        })
        .into_string()
    }

    fn css(&self) -> String {
        let heading_color = &self.heading_color.to_rgb_string();
        let image = &self.background_image;

        format!(
            r#"
.basic4-content-section{{
  padding-top: 5em;
  background-image: url("{image}");
  background-size: 100%;
  background-position: center center;
  background-repeat: no-repeat;
  height: 50vh;
  font-family: "Univers-Regular";
}}

.basic4-content-section #{CONTENT_HEADING} {{
  color: {heading_color} !important;
  font-size: 1.2em;
  text-align: center;
  text-shadow: 1px 1px 1px #000;
  opacity: .8;
}}

@media only screen and (max-width: 434px){{
  .basic4-content-section{{
    padding: 1em;
  }}

  .basic4-content-section{{
    padding-top: 1em;
    height: 20vh; 
}}

.basic4-content-section #{CONTENT_HEADING} {{
  font-size: 1em;
}}

}}
"#
        )
    }
}
