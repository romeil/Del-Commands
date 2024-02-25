use std::fs;
use std::path::Path;

pub fn remove_file(result: &String) -> () {
    let path = Path::new(result);
    if path.exists() {
        fs::remove_file(result)
            .expect("File delete failed.");
        println!("File deleted successfully!");
    } else {
        println!("File does not exist.");
    }
}

pub fn remove_folder(result: &String) -> () {
    let path = Path::new(result);
    if path.exists() {
        fs::remove_dir(result)
            .expect("Folder delete failed.");
        println!("Folder deleted successfully!");
    } else {
        println!("Folder does not exist.");
    }
}