use crate::structs::{CommandEnum, ParentStruct};

mod structs;

fn main() -> std::io::Result<()> {
    println!("Program start");
    use std::time::Instant;
    let now_all = Instant::now();
    let now = Instant::now();
    let current_dir = std::env::current_dir()?;
    println!("current_dir: {:?}", current_dir);
    let mut arr: Vec<std::fs::DirEntry> = vec![];
    read_files(current_dir, &mut arr);
    let mut elapsed = now.elapsed();
    println!("Elapsed find: {:.2?}", elapsed);
    let now = Instant::now();
    let mut files_arr: Vec<ParentStruct> = vec![];
    for i in arr {
        let file = std::fs::File::open(i.path()).unwrap();
        let mut yaml_struct: ParentStruct =
            serde_yaml::from_value(serde_yaml::from_reader(file).unwrap()).unwrap();
        yaml_struct.conf.path = Some(i.path());
        // println!("{:?}", yaml_struct);
        files_arr.push(yaml_struct);
    }
    elapsed = now.elapsed();
    println!("Elapsed parse: {:.2?}", elapsed);
    let now = Instant::now();
    for i in files_arr {
        let current_path = i.conf.path.as_ref().unwrap();
        let commands = match i.conf.commands {
            CommandEnum::Vec(vec) => vec,
            CommandEnum::String(string_command) => {
                let vec = vec![string_command];
                vec
            }
        };
        for j in commands {
            println!("{:?}", std::process::Command::new("cmd")
                .args(["/C", j.as_str()])
                .current_dir(current_path.parent().unwrap())
                .output()
                .expect("command error")
                .status)
        }
    }
    elapsed = now.elapsed();
    println!("Elapsed run commands: {:.2?}", elapsed);
    elapsed = now_all.elapsed();
    println!("Elapsed all time: {:.2?}", elapsed);
    println!("Program finish");
    Ok(())
}

fn read_files(
    directory: std::path::PathBuf,
    arr: &mut Vec<std::fs::DirEntry>,
) -> &Vec<std::fs::DirEntry> {
    for entry in std::fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let metadata = std::fs::metadata(&path).unwrap();
        if metadata.is_dir() {
            read_files(path, arr);
        } else if entry.file_name() == "rock.yaml" {
            arr.push(entry);
        }
    }
    arr
}
