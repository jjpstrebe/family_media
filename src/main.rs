use walkdir::{DirEntry, WalkDir};
use std::fs;


fn is_image(entry: &DirEntry) -> bool {

    if entry.metadata().unwrap().is_file() {
        infer::is_image(&fs::read(entry.path()).unwrap())
    } else {
        false
    }
    
}


fn is_image2(entry: &DirEntry) -> bool {

    if entry.metadata().unwrap().is_file() {
        let kind = infer::get_from_path(entry.path())
            .expect("file read successfully");
//            .expect("file type is known");
        match kind {
            Option => false,
            _ => kind.mime_type() == "image/jpeg",
        }
    } else {
        false
    }
    
}


fn is_video(entry: &DirEntry) -> bool {

    if entry.metadata().unwrap().is_file() {
        infer::is_video(&fs::read(entry.path()).unwrap())
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
    for entry in WalkDir::new("/Users/jjps/Downloads/test").into_iter().filter_map(|e| e.ok()) {
        if is_image2(&entry) {
            println!("{}", entry.path().display());
        }
    }

}
