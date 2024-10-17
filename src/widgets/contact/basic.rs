use crate::widgets::Widget;
use csscolorparser::Color;
use maud::html;

pub struct ContactWidget {
    pub background_color: Color,
    pub phone: String,
    pub website: String,
    pub email: String,
    pub postal: String,
}

impl Widget for ContactWidget {
    fn html(&self) -> String {
        html!(section #ContactWidget {
            div .columns{
                div .col-3.col-sm-12.contact-containers{
                    h4{i .icon.icon-apps{}}
                    h5{"Phone"}
                    h5 {(self.phone)}
                }
                div .col-3.col-sm-12.contact-containers{
                    h4{i .icon.icon-search{}}
                    h5{"Website"}
                    h5 {(self.website)}
                }
                div .col-3.col-sm-12.contact-containers{
                    h4{i .icon.icon-mail{}}
                    h5{"Email"}
                    h5 {(self.email)}
                }
                div .col-3.col-sm-12.contact-containers{
                    h4{i .icon.icon-edit{}}
                    h5{"Postal"}
                    h5 {(self.postal)}
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
