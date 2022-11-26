use mediamess::{self, MediaType};
use std::path::PathBuf;
use std::{env, fs, io};

fn get_target_folder() -> PathBuf {
    let home_folder = env::var("HOME").unwrap();
    let target_folder = [home_folder, String::from("Downloads")].iter().collect();
    target_folder
}

fn main() -> io::Result<()> {
    // check if target folder exists?
    let source_folder = get_target_folder();
    println!(
        "{:?} exists ? {:?}",
        &source_folder,
        &source_folder.exists()
    );

    // check if media folders already exist? Otherwise create them
    let target_folder = PathBuf::from("");

    // need only files from source folder
    let file_paths: Vec<PathBuf> = fs::read_dir(source_folder)?
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_ok())
        .filter(|e| e.file_type().unwrap().is_file())
        .map(|e| e.path())
        .collect();

    for p in file_paths.iter() {
        println!("{:?}", p)
    }

    let mediatypes = [MediaType::Image, MediaType::Gif, MediaType::Video];

    for mediatype in mediatypes {
        let paths = mediamess::select_by_mediatype(&file_paths, mediatype);

        // header
        println!("{:?}:", mediatype);

        // all apths
        for path in paths.iter() {
            //let rebased_path = mediamess::rebase_path_root(path, &target_folder);
            //println!("{:?} -> {:?}", path, rebased_path);
            println!(" - {:?}", path);
        }
    }

    Ok(())
}
