use std::fs;
use std::{path::PathBuf, process::exit};

use colored::Colorize;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

use crate::compilers::toolchain::Toolchain;
use crate::compilers::ts::TsCompiler;

pub(crate) enum ProjectLanguages {
    Js,
    Ts,
}

pub(crate) struct LanguagesVisitor;

impl<'de> Visitor<'de> for LanguagesVisitor {
    type Value = ProjectLanguages;
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "js" => Ok(ProjectLanguages::Js),
            "ts" => Ok(ProjectLanguages::Ts),
            _ => Err(E::custom(format!(
                "Wrong languages selected: provided '{}', but expecting 'js' or 'ts'",
                v
            ))),
        }
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("programming languages such as ts, js")
    }
}

impl Serialize for ProjectLanguages {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Js => serializer.serialize_str("js"),
            Self::Ts => serializer.serialize_str("ts"),
        }
    }
}

impl<'de> Deserialize<'de> for ProjectLanguages {
    fn deserialize<D>(deserializer: D) -> Result<ProjectLanguages, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(LanguagesVisitor)
    }
}

impl ProjectLanguages {
    pub(crate) fn vec_to_string(languages: &Vec<ProjectLanguages>) -> String {
        let strings: Vec<String> = languages
            .iter()
            .map(|language| match language {
                ProjectLanguages::Js => String::from("JavaScript"),
                ProjectLanguages::Ts => String::from("TypeScript"),
            })
            .collect();
        strings.join(", ")
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Project {
    pub mod_name: String,
    pub author: String,
    #[serde(skip_serializing)]
    #[serde(default)]
    pub path: PathBuf,
    pub languages: Vec<ProjectLanguages>,
}

impl PartialEq for ProjectLanguages {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Project {
    pub fn print_info(&self) {
        println!("{}", "Project information:".bold().green());
        println!("{}: {}", "Author".green(), self.author.yellow().bold());
        println!("{}: {}", "Mod name".green(), self.mod_name.yellow().bold());
        print!("{}: ", "Languages".green());
        println!("{}", ProjectLanguages::vec_to_string(&self.languages).yellow().bold());
    }

    pub fn build(&self) {
        println!("{}", "Building the project...".blue());

        let build_dir = self.path.clone().join("build");
        if !build_dir.is_dir() {
            fs::create_dir(build_dir).unwrap_or_else(|_| {
                println!("{}", "Error: cannot create a build folder".red());
                exit(-1);
            });
        }

        let toolchain = Toolchain::get(self);

        let toolchain = toolchain.unwrap_or_else(|| {
            println!("{}", "Error: a toolchain is required to build the project".red());
            exit(-1);
        });

        let ts_compiler = TsCompiler::get(&toolchain);

        self.languages.iter().for_each(|i| match i {
            ProjectLanguages::Ts => ts_compiler.build_project(self),
            _ => {}
        });

        println!("{}", "The build was completed saccessfully!".green().bold());
    }
}
