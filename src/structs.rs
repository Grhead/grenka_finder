use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum CommandEnum {
    Vec(Vec<String>),
    String(String),
}

#[derive(serde::Deserialize, Debug)]
pub struct ParentStruct {
    pub conf: ConfStruct,
}

#[derive(serde::Deserialize, Debug)]
pub struct ConfStruct {
    pub commands: CommandEnum,
    timeout: Option<u64>,
    rerun_counts: Option<u16>,
    weight: Option<u32>,
    pub path: Option<std::path::PathBuf>,
}

#[derive(Parser, Debug)]
#[command(author, version)]
pub struct Args {
    #[arg(short = 'c', long = "command", required = false)]
    pub command: Option<Vec<String>>,
    #[arg(short = 't', long = "timeout", required = false)]
    pub timeout: Option<u64>,
    #[arg(short = 'q', long = "quantity", required = false)]
    pub quantity: Option<u16>,
    #[arg(short = 'w', long = "weight", required = false)]
    pub weight: Option<u32>,
    #[arg(short = 'p', long = "path", required = false)]
    pub path: Option<std::path::PathBuf>,
}

pub trait PbInit {
    fn set_default_style(&self);
}

pub struct Progress {
    pub progress: ProgressBar,
}

impl Progress {
    pub fn new(length: u64) -> Progress {
        Progress {
            progress: ProgressBar::new(length),
        }
    }
}

impl PbInit for Progress {
    fn set_default_style(&self) {
        self.progress.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{bar:60.cyan/blue}] ({pos}/{len})",
            )
            .unwrap(),
        )
    }
}
