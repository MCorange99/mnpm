use std::io::{self, Write};
use whoami;

pub fn get(mut opt: crate::options::Options) -> crate::options::Options {
    let top_dir = &std::fs::canonicalize(opt.path.clone()).unwrap().file_name().unwrap().to_string_lossy().to_string();
    //name
    print!("Name: ({})", top_dir.as_str()); io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut opt.package.name).unwrap();
    opt.package.name = opt.package.name.trim().to_string();
    if opt.package.name == "" {
        opt.package.name = top_dir.to_string();
    }

    //version
    print!("Version: (0.0.1)"); io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut opt.package.version).unwrap();
    opt.package.version = opt.package.version.trim().to_string();
    if opt.package.version == "" {
        opt.package.version = "0.0.1".into();
    }

    //author
    let username = whoami::username();
    print!("Author: ({})", username); io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut opt.package.author).unwrap();
    opt.package.author = opt.package.author.trim().to_string();
    if opt.package.author == "" {
        
        opt.package.author = username.to_string();
    }

    //license
    print!("License: (MIT)"); io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut opt.package.license).unwrap();
    // TODO: Add a check if its a valid license.
    opt.package.license = opt.package.license.trim().to_string();
    if opt.package.license == "" {
        opt.package.license = "MIT".into();
    }

    //description
    print!("Description: (\"\")"); io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut opt.package.description).unwrap();
    opt.package.description = opt.package.description.trim().to_string();
    // * Unneeded
    // if opt.package.author == "" {
    //     opt.package.author = "".into();
    // }

    // TODO: Ask if the provided info is good.
    // Ask if input is ok
//     println!(
// "{{
//     \"name\": \"{name}\",
//     \"version\": \"{version}\",
//     \"description\": \"{description}\",
//     \"main\": \"{main}\",
//     \"scripts\": {{
//         \"test\": \"echo \\\"Error: no test specified\\\" && exit 1\"
//     }},
//     \"author\": \"{author}\",
//     \"license\": \"{license}\"
// }}",
// name=opt.package.name,
// version=opt.package.version,
// description=opt.package.description,
// main=opt.package.main,
// author=opt.package.author,
// license=opt.package.license
// );

    opt
}