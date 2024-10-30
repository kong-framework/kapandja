pub use contact::basic::ContactWidget;
pub use content::basic::ContentBasic;
pub use content::basic2::ContentBasic2;
pub use content::basic3::ContentBasic3;
pub use content::services::{ServiceItem, ServicesWidget};
pub use forms::contact::ContactFormWidget;
pub use hero::basic::HeroBasic;
pub use hero::basic2::HeroBasic2;
pub use lists::{ListBasic, ListItem};
pub use map::{MapWidget, MapWidgetMarker};
pub use nav::basic::NavBasic;
pub use nav::NavItem;

mod contact;
mod content;
mod forms;
mod hero;
mod lists;
mod map;
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
