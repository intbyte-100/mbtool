use std::{
    fs,
    path::{PathBuf},
};

use colored::Colorize;

use crate::{project::Project, util::request_yes_or_no};

pub struct Toolchain {
    path: PathBuf,
}

impl Toolchain {
    #[inline(always)]
    fn configure_toolchain(path: PathBuf) -> Option<Toolchain> {
        if let Err(e) = fs::create_dir(&path) {
            println!(
                "{}\n\t{}",
                "Error: cannot to configure the toolchain. Error message:".red(),
                e.to_string().red()
            );
            return None;
        }

        Some(Toolchain { path })
    }

    pub(crate) fn get(project: &Project) -> Option<Toolchain> {
        let toolchain_path = project.path.clone().join("build").join("toolchain");

        if !toolchain_path.is_dir() {
            println!("{}", "Warning: toolchain is not configured".yellow());

            match request_yes_or_no("Configure the toolchain for this project?") {
                true => return Self::configure_toolchain(toolchain_path),
                false => return None,
            }
        }

        Some(Toolchain {
            path: toolchain_path,
        })
    }
}
