use crate::askama_escaper;
use askama::Template;

#[derive(Template)]
#[template(path = "latex/autumn_leaves/section.tex", syntax = "tex")]
pub struct Section<'a> {
    pub icon: &'a str,
    pub title: &'a str,
}
