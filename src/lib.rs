#![warn(missing_docs)]

//! Help to extract any data from files

use std::path::Path;

///Check file for extract from them
pub fn check_file(file_path: &String) -> Result<(), String> {
    if Path::exists(Path::new(file_path)) {
        let pt = Path::new(file_path);
        println!("{:#?}", pt.metadata().expect("msg"));
        Ok(())
    } else {
        Err(format!("path is not valid, path: \"{}\"", file_path))
    }
}

fn is_file_archive(_file_path: &String) -> bool {
    true
}
