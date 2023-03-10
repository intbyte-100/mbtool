use std::fs::{self, File};

use colored::Colorize;

use crate::{Project, ProjectLanguages};

use std::io::Write;

pub(crate) fn generate_project(project: &Project) {
    println!("{}", "Starting project generation...".yellow());

    let mut src_dir = project.path.clone();
    src_dir.push("src");

    fs::create_dir(&src_dir).unwrap();

    //create the main file with code
    {
        let mut main = src_dir.clone();

        if project.languages.contains(&ProjectLanguages::Ts) {
            main.push("main.ts");
        } else {
            main.push("main.js")
        }

        let mut main = File::create(main).unwrap();

        write!(main, "console.log('Hello, world');").unwrap();
    }

    //create .gitignore
    {
        let mut gitignore = src_dir.clone();
        gitignore.push(".gitignore");
        let mut gitignore = File::create(gitignore).unwrap();
        write!(gitignore, "build").unwrap();
    }

    //generate config
    {
        let mut config = project.path.clone();
        config.push("modconfig.json");
        let mut config = File::create(config).unwrap();

        write!(
            config,
            "{}",
            serde_json::to_string_pretty(&project).unwrap()
        )
        .unwrap();
    }
}
