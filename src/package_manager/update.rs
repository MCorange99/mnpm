
pub fn run(opt: crate::options::Options) -> crate::options::Options{
    let x: Vec<&str> = opt.packagemanager.update.split(" ").collect();

    let _ = std::process::Command::new(x[0])
        .arg(x[1])
        .spawn();
    opt
}