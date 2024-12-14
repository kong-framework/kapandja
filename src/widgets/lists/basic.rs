use crate::widgets::Widget;
use csscolorparser::Color;
use maud::html;

use super::ListItem;

pub struct ListBasic {
    pub heading: String,
    pub heading_color: Color,
    pub background_color: Color,
    pub content: Vec<ListItem>,
    pub image: String,
    pub image_border_color: Color,
}

impl Widget for ListBasic {
    fn html(&self) -> String {
        html!(section .basic2-conent-section {
            div .columns{
                div .col-4.col-sm-12{
                    img src=(self.image) .img-responsive{}
                }
                div .col-1{}
                div .col-7.col-sm-12{
                    h5 #ListBasic {(self.heading)}
                    @for item in &self.content {
                        li { (item.text) }
                    }
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
  padding: 4em;
  padding-bottom: 4em;
  background-color: {background_color};
  font-family: "Univers-Regular";
}}

.basic2-conent-section #ListBasic {{
  color: {heading_color} !important;
}}

.basic2-conent-section img{{
  border-top-left-radius: 50px;
  border-bottom-right-radius: 50px;
  border: thick solid {image_border_color};
}}

@media only screen and (max-width: 434px){{
  .basic2-conent-section{{
    padding: 1em;
  }}
  
}}
"#
        )
    }
}
