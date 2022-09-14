// pub enum  {
    
// }

#[derive(Debug)]
#[derive(Clone)]

pub struct Package {
    pub name: String,
    pub main: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub license: String
}

#[derive(Debug)]
#[derive(Clone)]

pub struct File {
    pub content: String,
    pub path: String
}

#[derive(Debug)]
#[derive(Clone)]
pub struct PackageManager {
    pub update: String,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Options {
    pub verbose: bool,
    pub exec_name: String,
    pub structure_name: String,
    pub path: String,
    pub folders: Vec<String>,
    pub files: Vec<File>,
    pub package: Package,
    pub packagemanager: PackageManager
}