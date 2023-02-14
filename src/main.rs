use walkdir::{DirEntry, WalkDir};
use std::fs;


fn is_image_slow(entry: &DirEntry) -> bool {

    if entry.metadata().unwrap().is_file() {
        infer::is_image(&fs::read(entry.path()).unwrap())
    } else {
        false
    }
    
}


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


fn is_hidden(entry: &DirEntry) -> bool {

    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)

}

fn get_directories(path: &str) {

    for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_dir() {
            println!("{}", file.path().display());
        }
    }

}


fn main() {

//    get_directories("/Users/jjps/Downloads/test");
// find out if there is video and picture directories
// if not create them and put all directories in the pictures one
// create all the pictures directories in the video directory if they don't exist
// do the same but the other way around
// find all videos and the pictures directly and move them to the videos directories
// Do the same but the other way around
// remove all empty directories
    for entry in WalkDir::new("/Users/jjps/Downloads/test").into_iter().filter_map(|e| e.ok()) {
        if is_video(&entry) {
            println!("{}", entry.path().display());
        }
    }

}
