#[macro_use]
extern crate failure;
extern crate notify;
extern crate zip;

mod error;
mod project;

use std::{
    io::{self, BufRead},
    path::Path,
};
use error::SkittyResult;
use project::Project;


pub fn watch(path: &AsRef<Path>) -> SkittyResult<()>
{
    let project = Project::from(path)?;

    // If the sketch file doesn't exist make it
    if !project.sketch_path.is_file() {
        project.git_to_sketch()?;
    }

    // If the git files are newer than the sketch file
    if !project.is_git_newer()? {
        println!("Git files are newer than  sketch files.");
        if get_user_confirm(&"Do you want to recreate the sketch file? [Y/n]")? {
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
            "" => return Ok(true),
            "y" => return Ok(true),
            "n" => return Ok(false),
            _ => {},
        }
    }
}
