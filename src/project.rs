use std::path::PathBuf;

use colored::Colorize;
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

pub(crate) enum ProjectLanguages {
    Js,
    Ts,
}

struct LanguagesVisitor;

impl<'de> Visitor<'de> for LanguagesVisitor {
    type Value = ProjectLanguages;
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "js" => Ok(ProjectLanguages::Js),
            "ts" => Ok(ProjectLanguages::Ts),
            _ => Err(E::custom(format!("Wrong languages selected: provided '{}', but expecting 'js' or 'ts'", v)))
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

impl Project {
    pub fn print_info(&self) {
        println!("{}", "Project information:".bold().green());
        println!("{}: {}", "Author".green(), self.author.yellow().bold());
        println!("{}: {}", "Mod name".green(), self.mod_name.yellow().bold());
        print!("{}: ", "Languages".green());
        println!(
            "{}",
            ProjectLanguages::vec_to_string(&self.languages)
                .yellow()
                .bold()
        );
    }
}

impl PartialEq for ProjectLanguages {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}
