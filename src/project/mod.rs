mod fs_tools;
mod zip_tools;

use std::{
    fs::metadata,
    path::{Path, PathBuf},
    sync::mpsc::channel,
    time::Duration,
};
use notify::{RecommendedWatcher, Watcher, RecursiveMode};

use error::SkittyResult;
use project::fs_tools::{make_absolute, make_sketch, get_dir};
use project::zip_tools::{dir_to_zip, zip_to_dir};

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

    pub fn git_to_sketch(&self) -> SkittyResult<()> {
        dir_to_zip(&self.git_path, &self.sketch_path)
    }

    pub fn sketch_to_git(&self) -> SkittyResult<()> {
        zip_to_dir(&self.sketch_path, &self.git_path)
    }

    pub fn watch_sketch_file(&self) -> SkittyResult<()> {
        let (tx, rx) = channel();
        let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;

        watcher.watch(&self.sketch_path, RecursiveMode::NonRecursive)?;

        // This is a simple loop, but you may want to use more complex logic here,
        // for example to handle I/O.
        loop {
            rx.recv()?;
            println!("Sketch file has changed");
            self.sketch_to_git()?;
            println!("Sketch file has been deconstructed");
        }
        Ok(())
    }
}
