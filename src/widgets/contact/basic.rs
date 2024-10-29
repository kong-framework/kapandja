use crate::widgets::Widget;
use csscolorparser::Color;
use maud::html;

pub struct ContactWidget {
    pub background_color: Color,
    pub phone: String,
    pub email: String,
    pub location: String,
}

impl Widget for ContactWidget {
    fn html(&self) -> String {
        html!(section #ContactWidget {
            div .columns{
                div .col-4.col-sm-12.contact-containers{
                    h4{i .fa.fa-phone{}}
                    h5{"Phone"}
                    h5 {(self.phone)}
                }
                div .col-4.col-sm-12.contact-containers{
                    h4{i .icon.icon-mail{}}
                    h5{"Email"}
                    h5 {(self.email)}
                }
                div .col-4.col-sm-12.contact-containers{
                    h4{i .fa.fa-map-marker{}}
                    h5{"Location"}
                    h5 {(self.location)}
                }
            }
        })
        .into_string()
    }

    fn css(&self) -> String {
        let background_color = &self.background_color.to_rgb_string();

        format!(
            r#"
#ContactWidget {{
  padding: 4em;
  background-color: {background_color};
  font-family: "Univers-Regular";
}}

.contact-containers{{
  text-align: center;
}}
"#
        )
    }
}
