use std;
use crate::utils::color::*;


pub static UNKNOWN_ARG: (i32, &str) = (1, "Unknown argument '{}'");
pub static NOT_ENOUGH_ARGS: (i32, &str) = (2, "Not enough arguments");
pub static STRUCTURE_UNKNOWN: (i32, &str) = (3, "The provided structure '{}' could not be found");
pub static STRUCTURE_EMPTY: (i32, &str) = (4, "The structure was not provided, please provide one");
pub static PATH_INVALID: (i32, &str) = (5, "The provided path '{}' could not be found");
pub static PATH_EMPTY: (i32, &str) = (6, "The path was not provided, please provide one");
pub static UNABLE_TO_CREATE_FILE: (i32, &str) = (7, "Could not create file '{}', file exists or insufficient permissions");
pub static UNABLE_TO_CREATE_FOLDER: (i32, &str) = (8, "Could not create folder '{}', folder exists or insufficient permissions");
#[allow(dead_code)] pub static UNABLE_TO_READ_FILE: (i32, &str) = (9, "Could not read file '{}', insufficient permissions");
pub static UNABLE_TO_READ_FOLDER: (i32, &str) = (10, "Could not read folder '{}', insufficient permissions");
pub static NON_EMPTY_FOLDER: (i32, &str) = (11, "Folder '{}' is not empty.");

pub fn throw(err: (i32, &str), arg: &str) -> ! {
    
    println!("{green}[{purp}{errc}{green}] {red}ERROR{green}: {rs}{msg}", 
        msg=err.1.replace("{}", arg),
        errc=err.0,
        red=RED,
        green=GREEN,
        rs=RESET,
        purp=VIOLET,
        // ul=UNDERLINE
    );

    std::process::exit(err.0);
}

pub fn _exit(err: (i32, &str)){
    std::process::exit(err.0);
}