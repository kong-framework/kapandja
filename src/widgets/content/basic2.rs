use super::selectors::CONTENT_HEADING;
use crate::widgets::Widget;
use csscolorparser::Color;
use maud::html;

pub struct ContentBasic2 {
    pub heading: String,
    pub heading_color: Color,
    pub background_color: Color,
    pub content: String,
    pub image: String,
    pub image_border_color: Color,
}

impl Widget for ContentBasic2 {
    fn html(&self) -> String {
        html!(section .basic2-conent-section {
            div .columns{
                div .col-6{
                    img src=(self.image) .img-responsive{}
                }
                div .col-1{}
                div .col-5{
                    h5 #(CONTENT_HEADING) {(self.heading)}
                    p {(self.content)}
                }
            }
        })
        .into_string()
    }

    fn css(&self) -> String {
        let heading_color = &self.heading_color.to_rgb_string();
        let background_color = &self.background_color.to_rgb_string();
        let image_border_color = &self.image_border_color.to_rgb_string();

        format!(
            r#"
.basic2-conent-section{{
  padding: 1em 4em;
  padding-bottom: 4em;
  background-color: {background_color};
  font-family: "Univers-Regular";
}}

.basic2-conent-section #{CONTENT_HEADING} {{
  color: {heading_color} !important;
}}

.basic2-conent-section img{{
  border-top-left-radius: 50px;
  border-bottom-right-radius: 50px;
  border: thick solid {image_border_color};
}}
"#
        )
    }
}
