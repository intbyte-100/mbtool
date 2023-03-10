mod generator;
mod project;
mod util;

use crate::project::Project;
use crate::project::ProjectLanguages;
use io::Write;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;

use colored::*;

use crate::util::input_string;

fn main() {
    let command = std::env::args().nth(1);

    if command.is_some() {
        let command = command.unwrap();

        match command.as_str() {
            "help" => help_command(),
            "new" => new_project(&project_name_from_args()),
            "build" => build_project(),
            "info" => get_project().print_info(),

            _ => {
                println!("{}", "Error: unknown command".red());
                help_command();
                exit(-1);
            }
        }
    }

    if std::env::args().len() == 1 {
        help_command();
    }
}

fn project_name_from_args() -> String {
    let arg_count = std::env::args().len();
    if arg_count < 3 {
        println!("{}", "Error: project name not specified".red());
        exit(-1);
    }
    std::env::args().nth(2).unwrap()
}

fn get_project() -> Project {
    let mut path = PathBuf::from("modconfig.json");

    if !path.is_file() {
        path = PathBuf::from(std::env::args().nth(2).unwrap_or_else(| | {
            println!("{}\n\t{}\n\t{}", 
            "Error: cannot find the project".red(), "modconfig.json is not exist in the current directory or the project is not selected by argument".red() , 
            "try going to the project directory and run 'mbtool build' or specife project manualy: 'mbtool build /path/to/project'".red());
            exit(-1);
        }));

        if !path.exists() {
            println!("{}",format!("Error: the directory '{} is not exists'!", path.to_str().unwrap()).red());
            exit(-1);
        }

        if path.is_dir() {
            path.push("modconfig.json");
        } else {
            println!("{}", format!("Error: '{}' is not directory!", path.to_str().unwrap()).red());
            exit(-1)
        }
    }

    let json = fs::read_to_string(path).unwrap_or_else(|_| {
        println!("{}", "Error: 'modconfig.json' is not exist!");
        exit(-1);
    });

    let project: Project = serde_json::from_str(json.as_str()).unwrap_or_else(|error| {
        println!(
            "{}\n\t{}",
            "Error: invalid modconfig.json: ".red(),
            error.to_string().red()
        );
        exit(-1);
    });
    project
}
fn help_command() {
    println!("Usage: mbtool <command>");
    println!("Commands:");
    println!("    help       Display this help message");
    println!("    new        Create a new project");
    println!("    build      Build the project");
    println!("    info       Print the project info");
}

fn new_project(name: &String) {
    let path = Path::new(name);

    if path.is_dir() {
        println!(
            "{}",
            format!("Error: project with name '{}' is already exist", name).red()
        );
        exit(-1);
    } else if path.is_file() {
        println!("{}", "Error: file with same name is already exist".red());
        exit(-1);
    }

    fs::create_dir(path).unwrap_or_else(|error| {
        println!("{}", "Error: cannot to create project".red());
        print!("{}", "\terror message: ".red());
        println!("{}", error.to_string().red());
        exit(-1);
    });

    println!("{}", "Welcome to DragonBE mod master!".bold().green());
    print!("{}", "First of all, enter your mod name: ".green());
    io::stdout().flush().unwrap();

    let mod_name = input_string();

    print!("{}", "Also please input your name: ".green());
    io::stdout().flush().unwrap();

    let author = input_string();

    println!("{}", "Now, select the languages for developing:".green());
    println!("{}", "\t1. JavaScript and TypeScript".bold().yellow());
    println!("{}", "\t2. only JavaScript".bold().yellow());

    let mut project_languages: Vec<ProjectLanguages> = Vec::new();

    loop {
        print!("{}", "Please enter the number of your choise: ".green());
        io::stdout().flush().unwrap();

        let languages = input_string();
        let number: i32 = languages.trim().parse().unwrap_or(0);

        match number {
            1 => {
                project_languages.push(ProjectLanguages::Js);
                project_languages.push(ProjectLanguages::Ts);
            }
            2 => project_languages.push(ProjectLanguages::Js),
            _ => {
                println!(
                    "{}",
                    format!("'{}' is incorrect choise!!!", languages.trim()).red()
                );
                continue;
            }
        }
        break;
    }

    let project = Project {
        mod_name: mod_name.trim().to_string(),
        path: fs::canonicalize(path).unwrap(),
        author: author.trim().to_string(),
        languages: project_languages,
    };

    project.print_info();

    loop {
        print!("{}", "Create this project? [y/n]: ".green());
        io::stdout().flush().unwrap();

        let asnwer = input_string();

        match asnwer.trim() {
            "y" => generator::generate_project(&project),
            "n" => fs::remove_dir(path).unwrap(),

            val => {
                println!("{}", format!("'{}' is incorrect choise!!!", val).red());
                continue;
            }
        }
        break;
    }
}

fn build_project() {}
