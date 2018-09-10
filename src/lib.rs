#[macro_use]
extern crate failure;
extern crate notify;
extern crate serde;
extern crate serde_json;
extern crate zip;

mod error;
mod project;

use std::{
    io::{self, BufRead},
    path::Path,
};
use error::{SkittyError, SkittyResult};
use project::Project;

pub fn extract(path: &AsRef<Path>) -> SkittyResult<()>
{
    let project = Project::from(path)?;
    project.sketch_to_git()
}

pub fn combine(path: &AsRef<Path>) -> SkittyResult<()>
{
    let project = Project::from(path)?;
    project.git_to_sketch()
}

pub fn watch(path: &AsRef<Path>) -> SkittyResult<()>
{
    let project = Project::from(path)?;

    // If the sketch file and dir don't exist, lost cause
    if !project.sketch_path.is_file() && !project.git_path.is_dir() {
        return Err(SkittyError::ProjectNotFound(path.as_ref().into()));
    }

    // If the git project doesn't exist make it
    if !project.git_path.is_dir() {
        println!("Creating git dir");
        project.sketch_to_git()?;
    }

    // Or if the sketch file doesn't exist make it
    else if !project.sketch_path.is_file() {
        println!("Creating sketch file");
        project.git_to_sketch()?;
    }

    // Or if they both existed but the git files are newer than the sketch file
    else if !project.is_git_newer()? {
        println!("Git files are newer than the sketch file.");
        if get_user_confirm("Do you want to recreate the sketch file? [y/n]")? {
            project.git_to_sketch()?;
        }
    }

    println!("Watching sketch file for changes" );
    project.watch_sketch_file()
}

fn get_user_confirm<T>(question: T) -> SkittyResult<bool>
    where T: AsRef<str>
{
    loop {
        println!("{}", question.as_ref());
        let stdin = io::stdin();
        let mut confirm = String::with_capacity(8);
        stdin.lock().read_line(&mut confirm)?;
        match confirm.to_lowercase().trim() {
            "y" => return Ok(true),
            "n" => return Ok(false),
            _ => {},
        }
    }
}
