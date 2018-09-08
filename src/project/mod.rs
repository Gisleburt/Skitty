mod fs_tools;
mod zip_tools;

use std::{
    path::{Path, PathBuf},
    fs::metadata,
};

use error::{SkittyError, SkittyResult};
use project::fs_tools::{make_absolute, make_sketch, get_dir};

pub struct Project {
    pub sketch_path: PathBuf,
    pub git_path: PathBuf,
}

impl Project {
    pub fn from<T>(path: T) -> SkittyResult<Project>
        where T: AsRef<Path>
    {
        let absolute_path = make_absolute(&path)?;
        Ok(Project {
            sketch_path: make_sketch(absolute_path.as_ref())?.into(),
            git_path: get_dir(absolute_path.as_ref())?.into(),
        })
    }

    pub fn is_git_newer(&self) -> SkittyResult<bool> {
        let sketch_mtime = metadata(&self.sketch_path)?.modified()?;
        let git_mtime = metadata(&self.git_path)?.modified()?;
        Ok(git_mtime > sketch_mtime)
    }
}
