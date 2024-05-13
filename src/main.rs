use std::time::Instant;

use clap::Parser;
use colored::Colorize;

use grenka_finder::{get_files_count, read_files};

use crate::structs::{Args, CommandEnum, ParentStruct, PbInit, Progress};

mod structs;

fn main() -> std::io::Result<()> {
    let console_args = Args::parse();
    let now_all = Instant::now();
    println!(
        "{}: {} {}!",
        "Enkosh".bold().underline().magenta(),
        "Program".bold(),
        "start".bold()
    );
    let current_dir = std::env::current_dir()?;
    println!(
        "Selected directory: {}",
        current_dir.to_string_lossy().to_string().underline()
    );

    let files_count = get_files_count(&current_dir);

    let pb = Progress::new(files_count);
    pb.set_default_style();

    let mut entries_arr: Vec<std::fs::DirEntry> = vec![];
    let mut config_arr: Vec<ParentStruct> = vec![];

    read_files(current_dir, &mut entries_arr, &pb.progress);

    for i in entries_arr {
        let file = std::fs::File::open(i.path()).unwrap();
        let mut yaml_struct: ParentStruct =
            serde_yaml::from_value(serde_yaml::from_reader(file).unwrap()).unwrap();
        yaml_struct.conf.path = Some(i.path());
        config_arr.push(yaml_struct);
    }
    for i in config_arr {
        let current_path = i.conf.path.as_ref().unwrap();
        let commands = match i.conf.commands {
            CommandEnum::Vec(vec) => vec,
            CommandEnum::String(string_command) => {
                let vec = vec![string_command];
                vec
            }
        };
        for commands_run in commands {
            if cfg!(target_os = "windows") {
                std::process::Command::new("cmd")
                    .args(["/C", commands_run.as_str()])
                    .current_dir(current_path.parent().unwrap())
                    .expect("command error")
                    .status
            } else {
                std::process::Command::new("sh")
                    .arg("-c")
                    .arg(commands_run.as_str())
                    .current_dir(current_path.parent().unwrap())
                    .expect("command error")
                    .status
            };
        }
    }
    let elapsed = now_all.elapsed();
    println!("Elapsed all time: {:.2?}", elapsed);
    println!("Program finish");
    Ok(())
}
