use crate::utils::error::*;

mod nodejs_basic;
mod ts_clike;

pub fn get_structure(mut opt: crate::options::Options) -> crate::options::Options {
    let structure_name = opt.structure_name.as_str();

    match structure_name {
        "nodejs_basic" | "nodejs" => {
            opt = nodejs_basic::get(opt);
        }
        "ts-clike" | "ts_clike" => {
            opt = ts_clike::get(opt);
        }
        "" => throw(STRUCTURE_EMPTY, structure_name),
        _ => throw(STRUCTURE_UNKNOWN, structure_name)
    }
    return opt
}