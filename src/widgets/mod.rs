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
        let html = self.html();
        let css = self.css();
        let js = self.js();
        (html, css, js)
    }
}
