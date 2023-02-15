use walkdir::{DirEntry, WalkDir};
use std::fs;
use std::path::Path;


#[allow(dead_code)]
fn is_image_slow(entry: &DirEntry) -> bool {

    if entry.metadata().unwrap().is_file() {
        infer::is_image(&fs::read(entry.path()).unwrap())
    } else {
        false
    }
    
}


#[allow(dead_code)]
fn is_image(entry: &DirEntry) -> bool {

    if entry.metadata().unwrap().is_file() {
        let kind_result = infer::get_from_path(entry.path())
            .expect("file read successfully");
        match kind_result {
            Some(kind) => kind.mime_type() == "image/jpeg" ||
                          kind.mime_type() == "image/png" ||
                          kind.mime_type() == "image/gif" ||
                          kind.mime_type() == "image/webp" ||
                          kind.mime_type() == "image/x-canon-cr2" ||
                          kind.mime_type() == "image/tiff" ||
                          kind.mime_type() == "image/bmp" ||
                          kind.mime_type() == "image/heif" ||
                          kind.mime_type() == "image/avif" ||
                          kind.mime_type() == "image/vnd.ms-photo" ||
                          kind.mime_type() == "image/vnd.adobe.photoshop" ||
                          kind.mime_type() == "image/vnd.microsoft.icon" ||
                          kind.mime_type() == "image/openraster",
            // The file type is unknonw.
            None => false,
        }
    } else {
        false
    }
    
}


#[allow(dead_code)]
fn is_video_slow(entry: &DirEntry) -> bool {

    if entry.metadata().unwrap().is_file() {
        infer::is_video(&fs::read(entry.path()).unwrap())
    } else {
        false
    }

}


fn is_video(entry: &DirEntry) -> bool {

    if entry.metadata().unwrap().is_file() {
        let kind_result = infer::get_from_path(entry.path())
            .expect("file read successfully");
        match kind_result {
            Some(kind) => kind.mime_type() == "video/mp4" ||
                          kind.mime_type() == "video/x-m4v" ||
                          kind.mime_type() == "video/x-matroska" ||
                          kind.mime_type() == "video/webm" ||
                          kind.mime_type() == "video/quicktime" ||
                          kind.mime_type() == "video/x-msvideo" ||
                          kind.mime_type() == "video/x-ms-wmv" ||
                          kind.mime_type() == "video/mpeg" ||
                          kind.mime_type() == "video/x-flv",
            // The file type is unknonw.
            None => false,
        }
    } else {
        false
    }



}


#[allow(dead_code)]
fn is_hidden(entry: &DirEntry) -> bool {

    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)

}


fn get_directories(path: &str) -> Vec<String> {

    let mut dirs: Vec<String> = Vec::new();
    for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_dir() {
            dirs.push(file.path().strip_prefix(path).unwrap().to_str().unwrap().to_string());
        }
    }
    dirs

}


fn main() {

    let root_dir = "/Users/jjps/Downloads/test";
// find out if there is video and picture directories
// if not create them and put all directories in the pictures one
// create all the pictures directories in the video directory if they don't exist

// do the same but the other way around
// find all videos and the pictures directly and move them to the videos directories
// Do the same but the other way around
// remove all empty directories
    for entry in WalkDir::new(root_dir).into_iter().filter_map(|e| e.ok()) {
        if is_video(&entry) {
            println!("{}", entry.path().display());
        }
    }

    println!("\n");
    let pictures_path = Path::new(root_dir).join("pictures");
    let videos_path = Path::new(root_dir).join("videos");
    if !videos_path.exists() {
        fs::create_dir(&videos_path).unwrap();
    }
    if !pictures_path.exists() {
        println!("creating pictures");
        fs::create_dir(&pictures_path).unwrap();
        // Move everything into it.
        for path in fs::read_dir(root_dir).unwrap() {
            let p = path.unwrap().path();
            if p.file_name().unwrap() != "videos" && p.file_name().unwrap() != "pictures" {
                let new_p = pictures_path.join(p.file_name().unwrap());
                if !new_p.exists() {
                    fs::rename(p, new_p).unwrap();
                }
            }
        }
    }
    let dirs = get_directories(pictures_path.to_str().unwrap());
    for dir in &dirs {
        let new_p = videos_path.join(dir);
        if !new_p.exists() {
            fs::create_dir_all(&new_p).unwrap();
        }
    }

}
