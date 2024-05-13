
use std::time::Instant;

use clap::Parser;
use colored::Colorize;

use grenka_finder::{get_files_count, parse_files, read_files};
use grenka_finder::structs::{Args, CommandEnum, ParentStruct, PbInit, Progress};

fn main() -> std::io::Result<()> {
    let console_args = Args::parse();
    let now_all = Instant::now();
    println!(
        "{} - {} {}!",
        "START".bold().underline(),
        "Enkosh".bold().magenta(),
        "program".bold(),
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
    read_files(current_dir, &mut entries_arr, &pb.progress);
    pb.finish_pb();
    let config_arr: Vec<ParentStruct> = parse_files(entries_arr);
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
                    .status()
            } else {
                std::process::Command::new("sh")
                    .arg("-c")
                    .arg(commands_run.as_str())
                    .current_dir(current_path.parent().unwrap())
                    .status()
            }.expect("command internal error");
        }
    }
    let elapsed = now_all.elapsed();
    println!("Elapsed all time: {:.2?}", elapsed);
    println!(
        "{} - {} {}!",
        "FINISH".bold().underline(),
        "Enkosh".bold().magenta(),
        "program".bold(),

    );
    Ok(())
}
