use crate::widgets::Widget;
use maud::{html, DOCTYPE};

pub struct Epandja {
    pub title: String,
    pub outdir: String,
    pub widgets: Vec<Box<dyn Widget>>,
    pub css: Option<String>,
    pub js: Option<String>,
}

impl Epandja {
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
                    title{(title)}
                    @if style != "" {
                        style{(style)}
                    }
                    @if wids.1 != "" {
                        style{(wids.1)}
                    }
                    body{(wids.0)}
                    @if script != "" {
                        script{(script)}
                    }
                    @if script != wids.2 {
                        script{(wids.2)}
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
            html_out = format!("out{}", export.0);
            css_out = format!("out{}", export.1);
            js_out = format!("out{}", export.2);
        }

        (html_out, css_out, js_out)
    }
}
