use crate::options;

pub fn get(mut opt: options::Options) -> options::Options {
    opt.packagemanager.update = "yarn install".to_string();

    opt.package.main = "./src/index.js".into();

    opt.folders = vec![
        "./src/".into()
    ];

    opt.files = vec![
        options::File{
            path:"./src/index.js".into(),
            content:"console.log(\"Henlo, World!\");\n".into()
        },
        options::File{
            path:"./package.json".into(),
            content:
"{
    \"name\": \"{name}\",
    \"version\": \"{version}\",
    \"description\": \"{description}\",
    \"main\": \"{main}\",
    \"scripts\": {
        \"test\": \"echo \\\"Error: no test specified\\\" && exit 1\"
    },
    \"author\": \"{author}\",
    \"license\": \"{license}\"
}".to_string()
        }

    ];
    return opt;
}