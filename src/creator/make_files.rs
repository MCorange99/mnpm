use std::fs::write;
use crate::utils::error::*;
pub fn make(opt: crate::options::Options) -> crate::options::Options {

    let path = std::fs::canonicalize(opt.path.clone()).unwrap();

    
    
    
    for mut file in opt.files.clone() {
        let fp = path.join(file.path);
        // replace placeholders
        
        file.content = file.content.replace("{name}", &opt.package.name);
        file.content = file.content.replace("{main}", &opt.package.main);
        file.content = file.content.replace("{version}", &opt.package.version);
        file.content = file.content.replace("{description}", &opt.package.description);
        file.content = file.content.replace("{author}", &opt.package.author);
        file.content = file.content.replace("{license}", &opt.package.license);

        match write(fp.clone(), file.content) {
            Ok(_r) => (),
            Err(_e) => {
                throw(UNABLE_TO_CREATE_FILE, fp.display().to_string().as_str())
            }
        };
    }

    opt
}