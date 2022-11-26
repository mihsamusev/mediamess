use file_format;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref EXTENSIONS: HashMap<&'static str, MediaType> = vec![
        ("jpg", MediaType::Image),
        ("jpeg", MediaType::Image),
        ("bmp", MediaType::Image),
        ("png", MediaType::Image),
        ("gif", MediaType::Gif),
        ("mov", MediaType::Video),
        ("mp4", MediaType::Video),
        ("avi", MediaType::Video)
    ]
    .into_iter()
    .collect();
}

fn move_path(from: &str, to: &str) -> io::Result<()> {
    // tests paths exist!
    fs::copy(from, to)?;
    fs::remove_file(from)?;
    Ok(())
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum MediaType {
    Image,
    Video,
    Gif,
}

// maybe a media type
pub fn classify_mediatype(path: &PathBuf) -> Option<MediaType> {
    // its possible not to find an extension
    match path.extension().and_then(OsStr::to_str) {
        Some(ext) => {
            let lower = ext.to_lowercase();
            match EXTENSIONS.get(lower.as_str()) {
                Some(e) => Some(*e),
                None => None,
            }
        }
        None => None,
    }
    // its possible that extension is not supported
}

pub fn is_mediatype(path: &PathBuf, mediatype: MediaType) -> bool {
    match classify_mediatype(path) {
        Some(t) => t == mediatype,
        None => false,
    }
}

pub fn select_by_mediatype(paths: &[PathBuf], mediatype: MediaType) -> Vec<PathBuf> {
    let paths: Vec<PathBuf> = paths
        .iter()
        .cloned()
        .filter(|f| is_mediatype(f, mediatype))
        .collect();

    paths
}

// filename stays, everything else is from new root
// assuming valid frlenames already
pub fn rebase_path_root(file: &PathBuf, root: &PathBuf) -> io::Result<PathBuf> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_classify_mediatype_some() {
        let examples = vec![
            ("1.bmp", MediaType::Image),
            ("1.png", MediaType::Image),
            ("1.jpg", MediaType::Image),
            ("1.jpeg", MediaType::Image),
            ("1.BMP", MediaType::Image),
            ("1.gif", MediaType::Gif),
            ("1.mov", MediaType::Video),
            ("1.mp4", MediaType::Video),
            ("1.MP4", MediaType::Video),
        ];

        for (path, expected) in examples {
            let buf = PathBuf::from(path);
            match classify_mediatype(&buf) {
                Some(t) => assert_eq!(t, expected),
                None => panic!(),
            }
        }
    }

    #[test]
    fn test_classify_mediatype_none() {
        let examples = vec!["1.bmps", "1"];

        for path in examples {
            let buf = PathBuf::from(path);
            assert!(classify_mediatype(&buf).is_none())
        }
    }

    #[test]
    fn test_select_images_from_path_collection() {
        let paths: Vec<PathBuf> = ["/root/1.bmp", "1.png", "/root/1.whaever", "/root/1.gif"]
            .into_iter()
            .map(|p| PathBuf::from(p))
            .collect();

        let image_paths = select_by_mediatype(&paths, MediaType::Image);
        let expected: Vec<PathBuf> = ["/root/1.bmp", "1.png"]
            .into_iter()
            .map(|p| PathBuf::from(p))
            .collect();
        assert_eq!(image_paths, expected);
    }
}
