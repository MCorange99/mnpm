// use std;

mod args;
mod options;
mod utils;
mod structures;
mod creator;
mod package_manager;

fn main() {
    let mut options = options::Options{
        verbose: false,
        path: "".into(),
        exec_name: "".into(),
        structure_name: "".into(),
        folders: Vec::new(),
        files: Vec::new(),
        package: options::Package { name: String::new(), main: String::new(), version: String::new(), description: String::new(), author: String::new(), license: String::new() },
        packagemanager: options::PackageManager { update: String::new() }
    };
    
    // Cli arg parsing
    options = args::parse(options);
    
    // getting the structure
    options = structures::get_structure(options);
    
    // getting user input
    options = utils::get_info::get(options);
    
    // create the structure
    options = creator::start(options);

    let _ = package_manager::update::run(options);
    // install the dependencies
    return 
}
