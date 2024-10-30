use super::selectors::CONTENT_HEADING;
use crate::widgets::Widget;
use csscolorparser::Color;
use maud::html;

pub struct ContentBasic3 {
    pub heading: String,
    pub heading_color: Color,
    pub background_color: Color,
    pub content: String,
    pub content_color: Color,
}

impl Widget for ContentBasic3 {
    fn html(&self) -> String {
        html!(section .basic3-conent-section {
            div .columns{
                div .col-10.col-sm-12{
                    h3 #(CONTENT_HEADING) {(self.heading)}
                    p {(self.content)}
                }
            }
        })
        .into_string()
    }

    fn css(&self) -> String {
        let heading_color = &self.heading_color.to_rgb_string();
        let background_color = &self.background_color.to_rgb_string();
        let content_color = &self.content_color.to_rgb_string();

        format!(
            r#"
.basic3-conent-section{{
  padding: 1em 4em;
  background-color: {background_color};
  font-family: "Univers-Regular";
}}

.basic3-conent-section #{CONTENT_HEADING} {{
  color: {heading_color} !important;
}}

.basic3-conent-section p{{
  color: {content_color} !important;
}}

@media only screen and (max-width: 434px){{
  .basic3-conent-section{{
    padding: 1em;
  }}  
}}
"#
        )
    }
}
