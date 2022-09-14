mod make_files;
mod make_folders;

pub fn start(mut opt: crate::options::Options) -> crate::options::Options{
    

    opt = make_folders::make(opt);
    opt = make_files::make(opt);
    return opt
}