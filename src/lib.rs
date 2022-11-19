use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

enum MediaType {
    Image,
    Gif,
    Video,
}

struct MediaFile {
    path: PathBuf,
    media: MediaType,
}

struct MediaFolder {
    image: PathBuf,
    gif: PathBuf,
    video: PathBuf,
}

fn move_path(from: &str, to: &str) -> io::Result<()> {
    // tests paths exist!
    fs::copy(from, to)?;
    fs::remove_file(from)?;
    Ok(())
}

fn collect_media_paths(folder: Vec<PathBuf>) -> Vec<MediaFile> {
    vec![]
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
