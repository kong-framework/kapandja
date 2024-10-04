pub mod nav;

pub trait Widget {
    fn html(&self) -> String {
        "".to_string()
    }
    fn css(&self) -> String {
        "".to_string()
    }
    fn js(&self) -> String {
        "".to_string()
    }

    fn export(&self) -> (String, String, String) {
        let mut html = "".to_string();
        let mut css = "".to_string();
        let mut js = "".to_string();

        let h: String = (self.html().clone()).to_string();
        html = format!("out{}", h);

        let c: String = (self.css().clone()).to_string();
        css = format!("out{}", c);

        let j: String = (self.js().clone()).to_string();
        js = format!("out{}", j);

        (html, css, js)
    }
}
