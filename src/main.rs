use packmedia::{self, MediaType};
use std::collections::HashMap;
use std::path::PathBuf;
use std::{fs, io};
use clap::Parser;

const IMAGE_FOLDER: &str = "img";
const VIDEO_FOLDER: &str = "vid";
const GIF_FOLDER: &str = "gif";

#[derive(Parser)]
struct Cli {
    /// folder where media files are to be found
    source_folder: PathBuf,

    /// dont move files just show what will be moved
    #[arg(short, long)]
    dry_run: bool,
}

struct Config {
    media_folders: HashMap<MediaType, &'static str>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            media_folders: vec![
                (MediaType::Image, IMAGE_FOLDER),
                (MediaType::Video, VIDEO_FOLDER),
                (MediaType::Gif, GIF_FOLDER),
            ]
            .into_iter()
            .collect(),
        }
    }
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let config = Config::default();

    // check if media folders already exist? Otherwise create the
    // need only filenames, stripped from paths
    let file_paths: Vec<PathBuf> = fs::read_dir(&args.source_folder)?
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_ok())
        .filter(|e| e.file_type().unwrap().is_file())
        .map(|e| e.path())
        .collect();

    // 
    for (&mediatype, &mediafolder) in config.media_folders.iter() {
        // build folder paths
        let mut new_media_path = PathBuf::from(&args.source_folder);
        new_media_path.push(mediafolder);

        if !new_media_path.exists() & !args.dry_run {
            fs::create_dir_all(&new_media_path)?;
        }

        // get paths fit for that folder
        let paths = packmedia::select_by_mediatype(&file_paths, mediatype);

        // header
        println!(
            "{:?}: {} files to be moved to {:?}:",
            mediatype,
            paths.len(),
            new_media_path
        );

        // all apths
        for filename in packmedia::truncate_basepath(&paths) {
            let mut new_path = new_media_path.clone();
            new_path.push(&filename);

            let mut old_path = args.source_folder.clone();
            old_path.push(&filename);

            println!(" - {:?} --> {:?}", old_path, new_path);

            if !args.dry_run {
                packmedia::move_path(old_path, new_path)?;
            }
        }
    }
    Ok(())
}
