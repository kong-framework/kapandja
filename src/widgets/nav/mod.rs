pub mod basic;

pub mod selectors {
    pub const ACTIVE: &str = "active_nav";
    pub const NAVLINK: &str = "navlink";
    pub const LOGO: &str = "LOGO";
}

pub struct NavItem {
    pub name: String,
    pub url: String,
    pub active: bool,
}
