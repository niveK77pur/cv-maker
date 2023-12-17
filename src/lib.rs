use std::process::Stdio;

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

use tracing::{error, info};

mod askama_escaper;
pub mod latex_template;

pub struct CVData {}

impl CVData {
    pub async fn compile_pdf() {
        let handle = Command::new("lualatex")
            .current_dir("/tmp")
            .arg("mylatexcv.tex")
            .stdout(Stdio::piped())
            .kill_on_drop(true)
            .spawn();
        match handle {
            Ok(mut child) => {
                match child.stdout.take() {
                    Some(out) => {
                        let mut lines = BufReader::new(out).lines();
                        while let Some(line) =
                            lines.next_line().await.expect("This works?")
                        {
                            info!("LuaLaTex Output: {line}")
                        }
                    }
                    None => {
                        error!("Failed to get stdout")
                    }
                }
                match child.wait().await {
                    Ok(c) => info!("LuaLaTex Exit Code: {c}"),
                    Err(e) => error!(
                        "Failed to spawn lualatex command:
                {e}"
                    ),
                }
            }
            Err(e) => {
                error!("Failed to spawn lualatex command: {e}")
            }
        }
    }
}
