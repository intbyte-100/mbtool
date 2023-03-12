


use crate::project::Project;

use super::nodejs::NodeJS;
use super::toolchain::{Toolchain};

pub struct TsCompiler {
    nodejs: NodeJS
}


impl TsCompiler {
    pub fn get(toolchain: &Toolchain) -> TsCompiler{
        TsCompiler { nodejs: NodeJS::get(toolchain) }
    }
    

    pub(crate) fn build_project(&self, project: &Project){

    }
}

