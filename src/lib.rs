#[macro_use]
extern crate failure;
extern crate notify;
extern crate zip;

mod error;
mod project;

use std::{
    io::stdin,
    path::Path,
};
use error::SkittyResult;
use project::Project;


pub fn watch(path: &AsRef<Path>) -> SkittyResult<()>
{
    let project = Project::from(path)?;
    if !project.is_git_newer()? {
        println!("Git files are newer than  sketch files.");
        if get_user_input(&"Do you want to recreate the sketch file? [Y/n]")? {
            project.git_to_sketch()?;
        }
    }
    project.watch_sketch_file()
}

fn get_user_input<T>(question: T) -> SkittyResult<bool>
    where T: AsRef<str>
{
    println!("{}", question.as_ref());
    let mut input = String::with_capacity(8);
    loop {
        stdin().read_line(&mut input)?;
        match input.as_str() {
            "y" => return Ok(true),
            "n" => return Ok(false),
            _ => {}
        }
    }
}
