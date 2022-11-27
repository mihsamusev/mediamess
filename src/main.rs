use mediamess::{self, MediaType};
use std::collections::HashMap;
use std::path::PathBuf;
use std::{env, fs, io};

struct Config {
    dry_run: bool,
    source_folder: PathBuf,
    media_folders: HashMap<MediaType, &'static str>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dry_run: true,
            source_folder: get_default_folder(),
            media_folders: vec![
                (MediaType::Image, "img"),
                (MediaType::Video, "vid"),
                (MediaType::Gif, "gif"),
            ]
            .into_iter()
            .collect(),
        }
    }
}

fn get_default_folder() -> PathBuf {
    let home_folder = env::var("HOME").unwrap();
    let target_folder = [home_folder, String::from("Downloads")].iter().collect();
    target_folder
}

fn main() -> io::Result<()> {
    // check if target folder exists?
    let config = Config::default();
    //println!("{}", config);

    // check if media folders already exist? Otherwise create them

    // need only filenames, stripped from paths
    let file_paths: Vec<PathBuf> = fs::read_dir(&config.source_folder)?
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_ok())
        .filter(|e| e.file_type().unwrap().is_file())
        .map(|e| e.path())
        .collect();

    // for p in file_paths.iter() {
    //     println!("{:?}", p)
    // }

    for (&mediatype, &mediafolder) in config.media_folders.iter() {
        // build folder path
        let mut new_media_path = PathBuf::from(&config.source_folder);
        new_media_path.push(mediafolder);

        // create dirs if they dont exist
        if !new_media_path.exists() {
            fs::create_dir_all(&new_media_path)?;
        }

        // get paths fit for that folder
        let paths = mediamess::select_by_mediatype(&file_paths, mediatype);

        // header
        println!(
            "{:?}: {} files to be moved to {:?}:",
            mediatype,
            paths.len(),
            new_media_path
        );

        // all apths
        for filename in mediamess::truncate_basepath(&paths) {
            //let rebased_path = mediamess::rebase_path_root(path, &target_folder);
            //println!("{:?} -> {:?}", path, rebased_path);
            let mut new_path = new_media_path.clone();
            new_path.push(&filename);

            let mut old_path = config.source_folder.clone();
            old_path.push(&filename);

            println!(" - {:?} --> {:?}", old_path, new_path);

            if !config.dry_run {
                mediamess::move_path(old_path, new_path)?;
            }
        }
    }
    Ok(())
}
