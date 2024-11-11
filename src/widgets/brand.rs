use crate::widgets::Widget;
use csscolorparser::Color;
use maud::html;

pub struct BrandWidget {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub tertiary_color: Color,
    pub hero_logo: String,
    pub brand_description: String,
    pub logo_section_logo: String,
    pub logo_section_logo2: String,
    pub typefaces: Vec<String>,
}

impl Widget for BrandWidget {
    fn html(&self) -> String {
        html!(section #KAPANDJA_BRAND_WIDGET {
            section #KAPANDJA_BRAND_WIDGET_HERO{
                h1 {"Brand Style"}
                span .col-2 {"Guidelines"}
                p {
                    (self.brand_description)
                }

                div{
                    img #KAPANDJA_BRAND_WIDGET_LOGO src=(self.hero_logo){}
                }
            }

            div .columns #KAPANDJA_BRAND_WIDGET_LOGO_SECTION{
                div .col-10{
                    h4 {"Logo"}

                    p {"Our logo is one of the most important things in our company."}
                }

                div .col {
                    img #KAPANDJA_BRAND_WIDGET_LOGO1 src= (self.logo_section_logo){}
                }

                div .col {
                    img #KAPANDJA_BRAND_WIDGET_LOGO1 src= (self.logo_section_logo2){}
                }
            }

            div  #KAPANDJA_BRAND_WIDGET_COLOR_SECTION{
                h4 {"Colors"}
                div .columns {
                    div .col-4 {
                        div .colorBoxes #KAPANDJA_BRAND_WIDGET_PRIMARY_COLOR{}
                        b {"Primary Color:" (self.primary_color)}
                    }

                    div .col-4 {
                        div .colorBoxes #KAPANDJA_BRAND_WIDGET_SECONDARY_COLOR{}
                        b {"Secondary Color:" (self.secondary_color)}
                    }

                    div .col-4 {
                        div .colorBoxes #KAPANDJA_BRAND_WIDGET_TERTIARY_COLOR{}
                        b {"Tertiary Color:" (self.tertiary_color)}
                    }
                }
            }

            div  #KAPANDJA_BRAND_WIDGET_TYPOGRAPHY_SECTION{
                h4 {"Typography"}

                div{
                    h5 {"Typefaces"}

                    @for typeface in &self.typefaces{
                        div style="font-family="typeface{
                            h6{(typeface)}
                            p {"ABCDEFGHIJKLMNOPQRSTUVWXYZ
                               abcdefghijklmnopqrstuvwxyz"
                            }
                        }
                    }

                }
            }

            div {
                h1{"TODO"}
                ul{
                    li{ "Add section about the company and its values"}
                    li{"Add business card section"}
                    li{"Add letter head section"}
                    li{"Add advert posters section"}
                }
            }
        }
        )
        .into_string()
    }

    fn css(&self) -> String {
        let primary_color = self.primary_color.to_rgb_string();
        let secondary_color = self.secondary_color.to_rgb_string();
        let tertiary_color = self.tertiary_color.to_rgb_string();

        format!(
            r#"
#KAPANDJA_BRAND_WIDGET{{
  font-family: "Univers-Regular";
}}

#KAPANDJA_BRAND_WIDGET #KAPANDJA_BRAND_WIDGET_HERO{{
  background-color: {secondary_color};
  padding: 2em;
}}

#KAPANDJA_BRAND_WIDGET_HERO h1{{
  font-size: 2em;
  color: {tertiary_color};
}}

#KAPANDJA_BRAND_WIDGET_HERO span{{
  color: {secondary_color};
  background: {primary_color};
  font-weight: bold;
  font-size: 3em;
  padding: .1em;
}}

#KAPANDJA_BRAND_WIDGET_HERO div{{
  text-align: right;
}}

#KAPANDJA_BRAND_WIDGET_HERO #KAPANDJA_BRAND_WIDGET_LOGO{{
  width: 150px;
}}

#KAPANDJA_BRAND_WIDGET_HERO p{{
  margin-top: 2em;
  color: {tertiary_color};
}}

#KAPANDJA_BRAND_WIDGET_LOGO_SECTION{{
  padding: 2em;
  margin-top: 2em;
}}

#KAPANDJA_BRAND_WIDGET_LOGO_SECTION img{{
  width: 200px;
}}

#KAPANDJA_BRAND_WIDGET_COLOR_SECTION{{
  margin-top: 2em;
  padding: 2em;
  background-color: {secondary_color};
  color: {tertiary_color};
}}

#KAPANDJA_BRAND_WIDGET_COLOR_SECTION .colorBoxes{{
  width: 100px;
  height: 100px;
  border: thin solid {tertiary_color};
}}

#KAPANDJA_BRAND_WIDGET_COLOR_SECTION #KAPANDJA_BRAND_WIDGET_PRIMARY_COLOR{{
  background: {primary_color}
}}

#KAPANDJA_BRAND_WIDGET_COLOR_SECTION #KAPANDJA_BRAND_WIDGET_SECONDARY_COLOR{{
  background: {secondary_color}
}}

#KAPANDJA_BRAND_WIDGET_COLOR_SECTION #KAPANDJA_BRAND_WIDGET_TERTIARY_COLOR{{
  background: {tertiary_color}
}}

#KAPANDJA_BRAND_WIDGET_COLOR_SECTION{{
  font-size: .7em;
}}

#KAPANDJA_BRAND_WIDGET_TYPOGRAPHY_SECTION{{
  margin-top: 2em;
  padding: 2em;
}}
"#
        )
    }
}
