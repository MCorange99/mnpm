use std::fs::create_dir;
use crate::utils::error::*;
pub fn make(opt: crate::options::Options) -> crate::options::Options {

    let path = std::fs::canonicalize(opt.path.clone()).unwrap();

    // Test if folder empty
    if path == std::env::current_dir().unwrap() {
        match path.read_dir() {
            Ok(mut r) => {
                if !r.next().is_none() {
                    throw(NON_EMPTY_FOLDER, path.display().to_string().as_str())
                }
            }
            Err(_e) => {
                throw(UNABLE_TO_READ_FOLDER, path.display().to_string().as_str())
            }
        }
    } 
    // create the folder
    else {
        match create_dir(path.clone()) {
            Ok(_res) => (),
            Err(_e) => {
                throw(UNABLE_TO_CREATE_FOLDER, path.display().to_string().as_str())//lol so dumb
            }
        };
    }


    
    for folder in opt.folders.clone() {
        let fp = path.join(folder);
        match create_dir(fp.clone()) {
            Ok(_res) => (),
            Err(_e) => {
                throw(UNABLE_TO_CREATE_FOLDER, fp.display().to_string().as_str())//lol
            }
        };
    }

    opt
}