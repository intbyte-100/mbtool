use std::path::PathBuf;

use colored::Colorize;

pub(crate) enum ProjectLanguages {
    Js,
    Ts,
}

pub(crate) struct Project {
    pub mod_name: String,
    pub  author: String,
    pub path: PathBuf,
    pub languages: Vec<ProjectLanguages>,
}

impl Project {
    pub fn print_info(&self) {
        println!("{}: {}", "Author".green(), self.author.yellow().bold());
        println!("{}: {}", "Mod name".green(), self.mod_name.yellow().bold());
        print!("{}: ", "Languages".green());

        for (index, i) in self.languages.iter().enumerate() {
            match i {
                ProjectLanguages::Js => print!("{}", "JavaScript".yellow().bold()),
                ProjectLanguages::Ts => print!("{}", "TypeScript".yellow().bold()),
            }

            if index < self.languages.len() - 1 {
                print!(", ")
            }
        }
        println!()
    }
}

impl PartialEq for ProjectLanguages {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}
