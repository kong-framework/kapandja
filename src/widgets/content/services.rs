use crate::widgets::Widget;
use csscolorparser::Color;
use maud::html;

pub struct ServiceItem {
    pub icon: String,
    pub text: String,
}

pub struct ServicesWidget {
    pub heading: String,
    pub heading_color: Color,
    pub background_color: Color,
    pub services: Vec<ServiceItem>,
}

impl Widget for ServicesWidget {
    fn html(&self) -> String {
        html!(section #ServicesContentWidget {
            div .columns{
                @for service in &self.services {
                    div .col-2.col-sm-4 {
                        div .card{
                            div .card-image{
                                img src=(service.icon) .img-responsive{}
                            }
                            div .card-header{
                                div .card-title .h6{
                                    (service.text)
                                }
                            }
                        }
                    }
                }
            }
        })
        .into_string()
    }

    fn css(&self) -> String {
        let heading_color = &self.heading_color.to_rgb_string();
        let background_color = &self.background_color.to_rgb_string();

        format!(
            r#"
#ServicesContentWidget{{
  padding: 4em;
  background-color: {background_color};
  font-family: "Univers-Regular";
}}

#ServicesContentWidget h5{{
  color: {heading_color} !important;
}}

#ServicesContentWidget .card{{
  background-color: {background_color};
  padding: .3em;
  border: none;
  text-align: center;
}}

#ServicesContentWidget .card img{{
  width: 64px;
  margin: 0 auto;
}}

#ServicesContentWidget .card .card-title{{
  color: {heading_color};
}}

@media only screen and (max-width: 434px){{
  #ServicesContentWidget .card img{{
    width: 34px;
  }}

  #ServicesContentWidget{{
    padding: 1em;
  }}

  .card-title{{
    font-size: .8em;
  }}
}}
"#
        )
    }
}
