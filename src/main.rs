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
    println!(
        "{}",
        latex_template::Title {
            email: "Hello#@test.com",
            phone: "1234567890",
            picture_width: 5.0,
            other: "This is me lol",
            name: "Tim Musterman",
            picture_path: "/home/kuni/Pictures/Beethoven_new_songs.png",
            occupation: "Busy lol"
        }
        .render()
        .unwrap_or("BORKED".into())
    );

    info!("Section");
    println!(
        "{}",
        latex_template::Section {
            icon: "🌳",
            title: "hello, world!"
        }
        .render()
        .unwrap_or("BORKED".into())
    );

    info!("Studies");
    println!(
        "{}",
        latex_template::Studies {
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
                }
            ],
        }
        .render()
        .unwrap_or("BORKED".into())
    );

    info!("Tools");
    println!(
        "{}",
        latex_template::Tools {
            tools: vec![
                "Tool 1".into(),
                "Tool 2".into(),
                "Tool 3".into(),
                "Tool 4".into(),
                "Tool 5".into(),
                "Tool 6".into(),
                "Tool 7".into(),
            ],
            break_points: vec![3, 5]
        }
        .render()
        .unwrap_or("BORKED".into())
    );

    info!("Works");
    println!(
        "{}",
        latex_template::Works {
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
            ]
        }
        .render()
        .unwrap_or("BORKED".into())
    );

    info!("Languages");
    println!(
        "{}",
        latex_template::Languages {
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
            ]
        }
        .render()
        .unwrap_or("BORKED".into())
    );
}
