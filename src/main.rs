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
}
