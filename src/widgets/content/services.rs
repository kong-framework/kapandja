use crate::widgets::Widget;
use csscolorparser::Color;
use maud::html;

pub struct ServicesWidget {
    pub heading: String,
    pub heading_color: Color,
    pub background_color: Color,
}

impl Widget for ServicesWidget {
    fn html(&self) -> String {
        html!(section #ServicesContentWidget {
            div .columns{
                div .col-2.col-sm-4 {
                    div .card{
                        div .card-image{
                            img src="./icons/ict.png" .img-responsive{}
                        }
                        div .card-header{
                            div .card-title .h6{
                                "Information Technology"
                            }
                        }
                    }
                }
                div .col-2.col-sm-4 {
                    div .card{
                        div .card-image{
                            img src="./icons/conversation.png" .img-responsive{}
                        }
                        div .card-header{
                            div .card-title .h6{
                                "Consultancy Services"
                            }
                        }
                    }
                }
                div .col-2.col-sm-4 {
                    div .card{
                        div .card-image{
                            img src="./icons/security.png" .img-responsive{}
                        }
                        div .card-header{
                            div .card-title .h6{
                                "Security & Protection"
                            }
                        }
                    }
                }
                div .col-2.col-sm-4 {
                    div .card{
                        div .card-image{
                            img src="./icons/truck.png" .img-responsive{}
                        }
                        div .card-header{
                            div .card-title .h6{
                                "Transport & Logistics"
                            }
                        }
                    }
                }
                div .col-2.col-sm-4 {
                    div .card{
                        div .card-image{
                            img src="./icons/brickwall.png" .img-responsive{}
                        }
                        div .card-header{
                            div .card-title .h6{
                                "Construction & Renovations"
                            }
                        }
                    }
                }
                div .col-2.col-sm-4 {
                    div .card{
                        div .card-image{
                            img src="./icons/management.png" .img-responsive{}
                        }
                        div .card-header{
                            div .card-title .h6{
                                "Property Development"
                            }
                        }
                    }
                }
                div .col-2.col-sm-4 {
                    div .card{
                        div .card-image{
                            img src="./icons/flash.png" .img-responsive{}
                        }
                        div .card-header{
                            div .card-title .h6{
                                "Electrical & Power Engineering"
                            }
                        }
                    }
                }
                div .col-2.col-sm-4 {
                    div .card{
                        div .card-image{
                            img src="./icons/cleaning.png" .img-responsive{}
                        }
                        div .card-header{
                            div .card-title .h6{
                                "Hygiene & Cleaning"
                            }
                        }
                    }
                }
                div .col-2.col-sm-4 {
                    div .card{
                        div .card-image{
                            img src="./icons/catering.png" .img-responsive{}
                        }
                        div .card-header{
                            div .card-title .h6{
                                "Food & Catering"
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
}}
"#
        )
    }
}
