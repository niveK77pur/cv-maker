use std::io::Write;

use askama::Template;
use cv_maker::latex_template;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "cv_maker=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("hello, web server!");

    info!("Title");
    let title = latex_template::Title {
        email: "Hello#@test.com",
        phone: "1234567890",
        picture_width: 5.0,
        other: "This is me lol",
        name: "Tim Musterman",
        picture_path: "/home/kuni/Pictures/Beethoven_new_songs.png",
        occupation: "Busy lol",
    }
    .render()
    .unwrap_or("BORKED".into());
    println!("{}", title);

    info!("Section");
    let section = latex_template::Section {
        icon: "🌳",
        title: "hello, world!",
    }
    .render()
    .unwrap_or("BORKED".into());
    println!("{}", section);

    info!("Studies");
    let studies = latex_template::Studies {
        studies: vec![
            latex_template::Study {
                about: "My first study!".into(),
                location: "School 1".into(),
                from: "2017".into(),
                to: "2019".into(),
            },
            latex_template::Study {
                about: "My second study!".into(),
                location: "School 2".into(),
                from: "2022".into(),
                to: "2022".into(),
            },
        ],
    }
    .render()
    .unwrap_or("BORKED".into());
    println!("{}", studies);

    info!("Tools");
    let tools = latex_template::Tools {
        tools: vec![
            "Tool 1".into(),
            "Tool 2".into(),
            "Tool 3".into(),
            "Tool 4".into(),
            "Tool 5".into(),
            "Tool 6".into(),
            "Tool 7".into(),
        ],
        break_points: vec![3, 5],
    }
    .render()
    .unwrap_or("BORKED".into());
    println!("{}", tools);

    info!("Works");
    let works = latex_template::Works {
        works: vec![
            latex_template::Work {
                title: "My first job".into(),
                about: Some("I was tasked with doing my work".into()),
                location: "Office".into(),
                tools: Some("This, That, Other".into()),
                from: "Q1 2021".into(),
                to: "Q3 2022".into(),
            },
            latex_template::Work {
                title: "My second job".into(),
                about: None,
                location: "Office".into(),
                tools: Some("This, That, Other".into()),
                from: "Q2 2022".into(),
                to: "Q3 2023".into(),
            },
            latex_template::Work {
                title: "My third job".into(),
                about: Some("I was tasked with doing my work".into()),
                location: "Office".into(),
                tools: None,
                from: "Q1 2021".into(),
                to: "Q1 2021".into(),
            },
            latex_template::Work {
                title: "My fourth job".into(),
                about: None,
                location: "Office".into(),
                tools: None,
                from: "Q3 2024".into(),
                to: "Q4 2024".into(),
            },
        ],
    }
    .render()
    .unwrap_or("BORKED".into());
    println!("{}", works);

    info!("Languages");
    let languages = latex_template::Languages {
        columns: 2,
        colsep: 2.0,
        languages: vec![
            latex_template::Language {
                language: "Language 1".into(),
                cefr_level: latex_template::CEFR::C1,
                mothertongue: true,
            },
            latex_template::Language {
                language: "Language 2".into(),
                cefr_level: latex_template::CEFR::C2,
                mothertongue: false,
            },
            latex_template::Language {
                language: "Language 3".into(),
                cefr_level: latex_template::CEFR::A1,
                mothertongue: false,
            },
            latex_template::Language {
                language: "Language 4".into(),
                cefr_level: latex_template::CEFR::B2,
                mothertongue: true,
            },
            latex_template::Language {
                language: "Language 5".into(),
                cefr_level: latex_template::CEFR::C1,
                mothertongue: false,
            },
        ],
    }
    .render()
    .unwrap_or("BORKED".into());
    println!("{}", languages);

    info!("Document");
    let mut contents = String::new();
    contents.push_str(&title);
    contents.push_str(&section);
    contents.push_str(&studies);
    let document = latex_template::MainDocument {
        paper_margin: Some(1.7),
        paper_hmargin: Some(5.2),
        paper_vmargin: None,
        color_accent: latex_template::LatexColor {
            model: "HTML".into(),
            spec: "FF7D8A".into(),
        },
        color_secondary: latex_template::LatexColor {
            model: "HTML".into(),
            spec: "FF7D8A".into(),
        },
        color_text: latex_template::LatexColor {
            model: "HTML".into(),
            spec: "FF7D8A".into(),
        },
        color_anti_text: latex_template::LatexColor {
            model: "HTML".into(),
            spec: "FF7D8A".into(),
        },
        contents,
    }
    .render()
    .unwrap_or("BORKED".into());
    println!("{}", document);

    let file_location = "/tmp/mylatexcv.tex";
    match std::fs::File::create(file_location) {
        Ok(mut file) => match file.write_all(document.as_bytes()) {
            Ok(()) => info!("Contents written to file: {file_location}"),
            Err(e) => panic!("Failed to write to file: {e}"),
        },
        Err(e) => panic!("Could not create file: {e}"),
    };
}
