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

pub fn move_path<P: AsRef<Path>>(from: P, to: P) -> io::Result<()> {
    // tests paths exist!
    fs::copy(from.as_ref(), to.as_ref())?;
    fs::remove_file(from.as_ref())?;
    Ok(())
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum MediaType {
    Image,
    Video,
    Gif,
}

// maybe a media type
pub fn classify_mediatype<P: AsRef<Path>>(path: P) -> Option<MediaType> {
    // its possible not to find an extension
    match path.as_ref().extension().and_then(OsStr::to_str) {
        Some(ext) => {
            let lower = ext.to_lowercase();
            EXTENSIONS.get(lower.as_str()).cloned()
        }
        None => None,
    }
    // its possible that extension is not supported
}

pub fn is_mediatype<P: AsRef<Path>>(path: P, mediatype: MediaType) -> bool {
    match classify_mediatype(path) {
        Some(t) => t == mediatype,
        None => false,
    }
}

pub fn select_by_mediatype(paths: &[PathBuf], mediatype: MediaType) -> Vec<PathBuf> {
    paths
        .iter()
        .cloned()
        .filter(|f| is_mediatype(f, mediatype))
        .collect()
}

// only truncates the base, without consideration that
// the flie exists
pub fn truncate_basepath(paths: &[PathBuf]) -> Vec<PathBuf> {
    paths
        .iter()
        .filter_map(|p| p.file_name().and_then(OsStr::to_str))
        .map(PathBuf::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_classify_mediatype_some() {
        let examples = vec![
            ("root/1.bmp", MediaType::Image),
            ("../root/1.png", MediaType::Image),
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
    fn select_images_from_path_collection() {
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

    #[test]
    fn truncate_basepaths_from_list_of_paths() {
        let paths: Vec<PathBuf> = ["/root/1.bmp", "1.png", "/root/whaever"]
            .into_iter()
            .map(|p| PathBuf::from(p))
            .collect();
        let expected = [
            PathBuf::from("1.bmp"),
            PathBuf::from("1.png"),
            PathBuf::from("whaever"),
        ];

        assert_eq!(truncate_basepath(&paths), expected)
    }
}
