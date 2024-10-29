pub use contact::basic::ContactWidget;
pub use content::basic::ContentBasic;
pub use content::basic2::ContentBasic2;
pub use content::services::{ServiceItem, ServicesWidget};
pub use hero::basic::HeroBasic;
pub use lists::{ListBasic, ListItem};
pub use nav::basic::NavBasic;
pub use nav::NavItem;

mod contact;
mod content;
mod hero;
mod lists;
mod nav;

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
