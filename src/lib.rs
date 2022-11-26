use file_format;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn move_path(from: &str, to: &str) -> io::Result<()> {
    // tests paths exist!
    fs::copy(from, to)?;
    fs::remove_file(from)?;
    Ok(())
}

pub enum MediaType {
    Image,
    Video,
    Gif,
}

// maybe a media type
pub fn classify_mediatype(file: &PathBuf) -> Option<MediaType> {
    todo!()
}

pub fn is_mediatype(file: &PathBuf, mediatype: MediaType) -> bool {
    true
}

pub fn select_images(files: &[PathBuf]) -> Vec<PathBuf> {
    // files
    //     .into_iter()
    //     .filter(|f| is_mediatype(f, MediaType::Image))
    //     .collect()
    todo!()
}

pub fn select_videos(files: Vec<PathBuf>) -> Vec<PathBuf> {
    todo!()
}

pub fn select_gifs(files: Vec<PathBuf>) -> Vec<PathBuf> {
    todo!()
}

// filename stays, everything else is from new root
// assuming valid frlenames already
pub fn rebase_path_root(file: &PathBuf, root: &PathBuf) -> io::Result<PathBuf> {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
