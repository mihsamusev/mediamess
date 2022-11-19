use glob::glob;
//use mediamess;
use std::path::PathBuf;
use std::{env, fs, io};

fn get_target_folder() -> PathBuf {
    let home_folder = env::var("HOME").unwrap();
    let target_folder = [home_folder, String::from("Downloads")].iter().collect();
    target_folder
}

fn main() -> io::Result<()> {
    let target_folder = get_target_folder();
    println!(
        "{:?} exists ? {:?}",
        &target_folder,
        &target_folder.exists()
    );

    let file_paths: Vec<PathBuf> = fs::read_dir(target_folder)?
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect();

    for p in file_paths {
        println!("{:?}", p)
    }

    Ok(())
}
