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
