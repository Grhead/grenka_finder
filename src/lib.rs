use std::thread;
use std::time::Duration;

use crate::structs::ParentStruct;

pub mod structs;

pub fn get_files_count(directory: &std::path::PathBuf) -> u64 {
    let mut counter: i32 = 0;
    *files_count(&directory, &mut counter) as u64
}

pub fn read_files<'a>(
    directory: std::path::PathBuf,
    arr: &'a mut Vec<std::fs::DirEntry>,
    pb: &'a indicatif::ProgressBar,
) -> &'a Vec<std::fs::DirEntry> {
    pb.inc(1);
    for entry in std::fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let metadata = std::fs::metadata(&path).unwrap();
        if metadata.is_dir() {
            read_files(path, arr, &pb);
        } else if entry.file_name() == "rock.yaml" {
            arr.push(entry);
        }
    }

    arr
}

fn files_count<'a>(directory: &'a std::path::PathBuf, counter: &'a mut i32) -> &'a i32 {
    for entry in std::fs::read_dir(directory).unwrap() {
        *counter += 1;
        if let Ok(entry) = entry {
            let path = entry.path();
            let metadata = std::fs::metadata(&path).unwrap();
            if metadata.is_dir() {
                files_count(&path, counter);
            }
        }
    }
    counter
}

pub fn parse_files(
    entries_arr: Vec<std::fs::DirEntry>,
    pb: &indicatif::ProgressBar,
) -> Vec<ParentStruct> {
    let mut config_arr: Vec<ParentStruct> = vec![];
    for i in entries_arr {
        let file = std::fs::File::open(i.path()).unwrap();
        let mut yaml_struct: ParentStruct =
            serde_yaml::from_value(serde_yaml::from_reader(file).unwrap()).unwrap();
        yaml_struct.conf.path = Some(i.path());
        config_arr.push(yaml_struct);
        pb.inc(1);
    }
    config_arr
}

pub fn run_command(
    current_path: &std::path::PathBuf,
    command: String,
    pb: &indicatif::ProgressBar,
) {
    if cfg!(target_os = "windows") {
        println!("windows");
        std::process::Command::new("cmd")
            .args(["/C", command.as_str()])
            .current_dir(current_path.parent().unwrap())
            .status()
    } else {
        println!("linux");
        std::process::Command::new("sh")
            .arg("-c")
            .arg(command.as_str())
            .current_dir(current_path.parent().unwrap())
            .status()
    }.expect("run command error");
    pb.inc(1);
}
