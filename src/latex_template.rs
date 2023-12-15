use crate::askama_escaper;
use askama::Template;
use getset::Getters;

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

#[derive(Template)]
#[template(path = "latex/autumn_leaves/works.tex", syntax = "tex")]
pub struct Works {
    pub works: Vec<Work>,
}

#[derive(Getters)]
#[getset(get)]
pub struct Work {
    pub title: String,
    pub about: Option<String>,
    pub location: String,
    pub tools: Option<String>,
    pub from: String,
    pub to: String,
}

#[derive(Template)]
#[template(path = "latex/autumn_leaves/languages.tex", syntax = "tex")]
pub struct Languages {
    pub columns: usize,
    pub colsep: f64,
    pub languages: Vec<Language>,
}

pub struct Language {
    pub language: String,
    pub cefr_level: CEFR,
    pub mothertongue: bool,
}

pub enum CEFR {
    A1,
    A2,
    B1,
    B2,
    C1,
    C2,
}

impl Language {
    pub fn max_level_int(&self) -> usize {
        6
    }
    pub fn current_level_int(&self) -> usize {
        match &self.cefr_level {
            CEFR::A1 => 1,
            CEFR::A2 => 2,
            CEFR::B1 => 3,
            CEFR::B2 => 4,
            CEFR::C1 => 5,
            CEFR::C2 => 6,
        }
    }
}

#[derive(Template)]
#[template(path = "latex/autumn_leaves/main_document.tex", syntax = "tex")]
pub struct MainDocument {
    pub paper_margin: Option<f64>,
    pub paper_hmargin: Option<f64>,
    pub paper_vmargin: Option<f64>,
    pub color_accent: LatexColor,
    pub color_secondary: LatexColor,
    pub color_text: LatexColor,
    pub color_anti_text: LatexColor,
    pub contents: String,
}

pub struct LatexColor {
    pub model: String,
    pub spec: String,
}
