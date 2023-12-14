use crate::askama_escaper;
use askama::Template;

#[derive(Template)]
#[template(path = "latex/autumn_leaves/section.tex", syntax = "tex")]
pub struct Section<'a> {
    pub icon: &'a str,
    pub title: &'a str,
}

#[derive(Template)]
#[template(path = "latex/autumn_leaves/title.tex", syntax = "tex")]
pub struct Title<'a> {
    pub email: &'a str,
    pub phone: &'a str,
    pub picture_width: f64,
    pub other: &'a str,
    pub name: &'a str,
    pub picture_path: &'a str,
    pub occupation: &'a str,
}
