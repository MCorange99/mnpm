use crate::options;

pub fn get(mut opt: options::Options) -> options::Options {

    opt.packagemanager.update = "yarn install".to_string();

    opt.package.main = "./target/index.js".into();

    opt.folders = vec![
        "./src/".into(),
        "./src/types".into(),
    ];

    opt.files = vec![
        options::File{
            path:"./src/index.ts".into(),
            content:"console.log(\"Henlo, World!\");\n".into()
        },
        options::File{
            path:"./loader.ts".into(),
            content:"export function main(argc: int, argv: str[]): int {\n    return 0;\n}\n".into()
        },
        options::File{
            path:"./src/types/index.d.ts".into(),
            content:"// Type definitions for {name}\n// Project: {name}\n// Definitions by: {author}\n\ndeclare global {\n    // yes\n    type str = string;\n    type int = number\n}\n/*~ If your module exports nothing, you\'ll need this line. Otherwise, delete it */\nexport {};\n".into()
        },
        options::File{
            path:"./.editorconfig".into(),
            content:"root = true\n\n[*]\nend_of_line = lf\ninsert_final_newline = true\n\n[*.{js,json,yml,ts}]\ncharset = utf-8\nindent_style = space\nindent_size = 4\n".into()
        },
        options::File{
            path:"./tsconfig.json".into(),
            content:"{\n    \"extends\": \"@tsconfig/recommended/tsconfig.json\",\n    \"compilerOptions\": {\n        \"typeRoots\": [\"./node_modules/@types\", \"./src/types\"],\n        \"moduleResolution\": \"NodeNext\",\n        \"outDir\": \"./target/\",\n        \"target\": \"ESNext\",\n        \"module\": \"ESNext\",\n        \"allowSyntheticDefaultImports\": true,\n        \"preserveConstEnums\": true,\n        \"noImplicitReturns\": true,\n        \"resolveJsonModule\": true,\n        \"removeComments\": true,\n        \"noImplicitAny\": true,\n        \"alwaysStrict\": true,\n        \"sourceMap\": true\n    },\n    \"include\": [\"src/**/*.ts\", \"loader.ts\"],\n    \"exclude\": [\"node_modules\", \"**/*.spec.ts\"]\n}".into()
        },
        options::File{
            path:"./.gitignore".into(),
            content:"target/\nnode_modules/\npackage-lock.json\n".into()
        },
        options::File{
            path:"./package.json".into(),
            content: "{\n    \"name\": \"tmp\",\n    \"version\": \"0.0.1\",\n    \"description\": \"\",\n    \"main\": \"./build/loader.js\",\n    \"scripts\": {\n        \"test\": \"echo \\\"Error: no test specified\\\" && exit 1\"\n    },\n    \"author\": \"mcorange\",\n    \"license\": \"MIT\",\n    \"dependencies\": {\n        \"@tsconfig/recommended\": \"^1.0.1\",\n        \"@types/node\": \"^18.7.18\",\n        \"@typescript-eslint/eslint-plugin\": \"^5.36.1\",\n        \"@typescript-eslint/parser\": \"^5.36.1\",\n        \"eslint\": \"^8.23.1\",\n        \"typescript\": \"^4.8.3\"\n    }\n}\n".to_string()
        }

    ];
    return opt;
}