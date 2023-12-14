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

#[derive(Template)]
#[template(path = "latex/autumn_leaves/studies.tex", syntax = "tex")]
pub struct Studies {
    pub studies: Vec<Study>,
}

pub struct Study {
    pub about: String,
    pub location: String,
    pub from: String,
    pub to: String,
}

#[derive(Template)]
#[template(path = "latex/autumn_leaves/tools.tex", syntax = "tex")]
pub struct Tools {
    pub tools: Vec<String>,
    pub break_points: Vec<usize>,
}

impl Tools {
    pub fn break_at(&self, entry: &usize) -> bool {
        return self.break_points.contains(&entry);
    }
}
