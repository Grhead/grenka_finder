
use std::time::Instant;

use clap::Parser;
use colored::Colorize;

use grenka_finder::{get_files_count, parse_files, read_files, run_command};
use grenka_finder::structs::{Args, CommandEnum, ParentStruct, PbInit, Progress};

fn main() -> std::io::Result<()> {
    let console_args = Args::parse();
    let now_all = Instant::now();
    println!(
        "{} - {} {}!\n",
        "START".bold().underline(),
        "Enkosh".bold().magenta(),
        "program".bold(),
    );
    let current_dir = std::env::current_dir()?;
    println!(
        "{}: {}\n",
        "Selected directory".italic(),
        current_dir.to_string_lossy().to_string().underline()
    );

    let files_count = get_files_count(&current_dir);

    let mut pb = Progress::new(files_count);
    pb.set_default_style("Read files".to_string());

    let mut entries_arr: Vec<std::fs::DirEntry> = vec![];
    read_files(current_dir, &mut entries_arr, &pb.progress);
    pb.finish_pb();
    pb = Progress::new(entries_arr.len() as u64);
    pb.set_default_style("Enkosh files count".to_string());
    let config_arr: Vec<ParentStruct> = parse_files(entries_arr, &pb.progress);
    pb.finish_pb();
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
            run_command(current_path, commands_run);
        }
    }

    let elapsed = now_all.elapsed();
    println!("\n\n{} {:.2?}", "Elapsed all time:".italic(), elapsed);
    println!(
        "{} - {} {}!",
        "FINISH".bold().underline(),
        "Enkosh".bold().magenta(),
        "program".bold(),

    );
    Ok(())
}
