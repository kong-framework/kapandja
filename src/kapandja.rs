use crate::widgets::Widget;
use maud::{html, PreEscaped, DOCTYPE};

pub struct Kapandja {
    pub title: String,
    pub file_path: std::path::PathBuf,
    pub widgets: Vec<Box<dyn Widget>>,
    pub css: Option<String>,
    pub js: Option<String>,
    pub leaflet: bool,
}

impl Kapandja {
    pub fn export(&self) -> std::io::Result<()> {
        println!("Exporting {} page to {:?}", self.title, self.file_path);

        let html = self.build();
        std::fs::write(&self.file_path, &html.as_bytes())?;
        Ok(())
    }

    pub fn build(&self) -> String {
        let title = &self.title;
        let style = if let Some(css) = &self.css { &css } else { "" };
        let wids = self.build_widgets();
        let script = if let Some(js) = &self.js { &js } else { "" };
        let out = html! {
            (DOCTYPE)
                head lang="en"{
                    meta charset="UTF-8";
                    meta name="viewport" content="width=device-width, initial-scale=1.0";
                    meta http-equiv="X-UA-Compatible" content="ie=edge";
                    link rel="stylesheet" href="css/vendor/spectre.min.css";
                    link rel="stylesheet" href="css/vendor/spectre-exp.min.css";
                    link rel="stylesheet" href="css/vendor/spectre-icons.min.css";

                    @if self.leaflet {
                        link rel="stylesheet"
                            href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css"
                            integrity="sha256-p4NxAoJBhIIN+hmNHrzRCf9tD/miZyoHS5obTRR9BMY="
                            crossorigin="";
                    }

                    @if self.font_awesome_icons {
                        link rel="stylesheet"
                            href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css"{}
                    }

                    title{(title)}
                    @if style != "" {
                        style{(PreEscaped(style))}
                    }
                    @if wids.1 != "" {
                        style{(PreEscaped(wids.1))}
                    }
                    body{(PreEscaped(wids.0))}
                    @if script != "" {
                        script{(PreEscaped(script))}
                    }
                    @if script != wids.2 {
                        script{(PreEscaped(wids.2))}
                    }
                }
        }
        .into_string();
        out
    }

    pub fn build_widgets(&self) -> (String, String, String) {
        let mut html_out = "".to_string();
        let mut css_out = "".to_string();
        let mut js_out = "".to_string();

        for wid in &self.widgets {
            let export: (String, String, String) = wid.export().clone();
            html_out = format!("{html_out}{}", export.0);
            css_out = format!("{css_out}{}", export.1);
            js_out = format!("{js_out}{}", export.2);
        }

        (html_out, css_out, js_out)
    }
}
