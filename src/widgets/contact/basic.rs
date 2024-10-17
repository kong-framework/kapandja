use crate::widgets::Widget;
use csscolorparser::Color;
use maud::html;

pub struct ContactWidget {
    pub heading: String,
}

impl Widget for ContactWidget {
    fn html(&self) -> String {
        html!(section #ContactWidget {
            h5 {(self.heading)}
        })
        .into_string()
    }

    fn css(&self) -> String {
        let heading_color = &self.heading_color.to_rgb_string();
        let background_color = &self.background_color.to_rgb_string();
        let image_border_color = &self.image_border_color.to_rgb_string();

        format!(
            r#"
.basic-conent-section{{
  padding: 4em;
  background-color: {background_color};
  font-family: "Univers-Regular";
}}

.basic-conent-section #{CONTENT_HEADING} {{
  color: {heading_color} !important;
}}

.basic-conent-section img{{
  border-top-left-radius: 50px;
  border-bottom-right-radius: 50px;
  border: thick solid {image_border_color};
}}
"#
        )
    }
}
