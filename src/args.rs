use std::env;
use crate::{utils::error};

pub fn parse(mut opt: crate::options::Options ) -> crate::options::Options {
    let mut structure_name: bool = false;


    let mut args: Vec<String> = env::args().collect();
    opt.exec_name = args.remove(0);

    if args.len() < 1 {
        crate::utils::usage::usage(&opt.exec_name);
        error::throw(error::NOT_ENOUGH_ARGS, "");
    }

    opt.path = args.pop().unwrap();
    
    if opt.path == "" {
        error::throw(error::PATH_EMPTY, opt.path.as_str());
    }
    if !std::path::Path::new(&opt.path).is_dir() {
        error::throw(error::PATH_INVALID, opt.path.as_str());
    }


    for arg in args {

        if structure_name {
            opt.structure_name = arg;
            continue;
        }


        match arg.as_str() {
            "--structure" | "-S" => {
                structure_name = true;
            } 
            "-h" | "--help" => {
                crate::utils::usage::usage(&opt.exec_name);
            }
            _ => {
                error::throw(error::UNKNOWN_ARG, arg.as_str());
            }
        }
    }

    return opt
}