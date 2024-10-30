use crate::widgets::Widget;
use csscolorparser::Color;
use maud::html;

pub struct ContactFormWidget {
    pub background_color: Color,
    pub color: Color,
    pub image: String,
    pub image_border_color: Color,
}

impl Widget for ContactFormWidget {
    fn html(&self) -> String {
        html!(section #KAPANDJA_CONTACT_FORM_WIDGET {
            div .columns{
                div .col-5.col-sm-12{
                    img src=(self.image) .img-responsive{}
                }
                div .col-1{}
                div .col-6.col-sm-12.form-group{
                    h4 {"Send us a message"}
                    label .form-label for="nameInput" {"Your Name"}
                    input .form-input type="text" id="nameInput" placeholder="Name"{}
                    label .form-label for="emailInput" {"Your Email Address"}
                    input .form-input type="text" id="emailInput" placeholder="Email"{}
                    label .form-label for="phoneInput" {"Your Phone Number"}
                    input .form-input type="text" id="phoneInput" placeholder="Phone"{}
                    label .form-label for="msgInput" {"Your Message"}
                    textarea .form-input id="msgInput"  placeholder="Message" rows="3"{}
                    br{}
                    button #SENDMSGBTN .btn {"Send message"}
                }

            }
        })
        .into_string()
    }

    fn css(&self) -> String {
        let background_color = &self.background_color.to_rgb_string();
        let color = &self.color.to_rgb_string();
        let image_border_color = &self.image_border_color.to_rgb_string();

        format!(
            r#"
#KAPANDJA_CONTACT_FORM_WIDGET {{
  background-color: {background_color};
  padding: 1em 4em;
  font-family: "Univers-Regular";
  color: {color};
}}

#KAPANDJA_CONTACT_FORM_WIDGET img{{
  border-top-left-radius: 50px;
  border-bottom-right-radius: 50px;
  border: thick solid {image_border_color};
}}

#SENDMSGBTN{{
  color: {background_color};
  border-color: {background_color};
}}
"#
        )
    }
}
