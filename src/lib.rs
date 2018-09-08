#[macro_use]
extern crate failure;
extern crate notify;
extern crate zip;

mod error;
mod zip_tools;

use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use std::sync::mpsc::channel;
use std::path::Path;
use std::fs::metadata;
use error::{SkittyError, SkittyResult};
use std::borrow::Cow;
use std::ffi::OsStr;


fn make_absolute<'a>(path: &'a AsRef<Path>) -> SkittyResult<Cow<'a, Path>>
{
    let path = path.as_ref();
    if path.is_absolute() {
        Ok(path.into())
    } else {
        Ok(std::env::current_dir()?.join(path).into())
    }
}

fn is_sketch_file<T>(path: T) -> SkittyResult<bool>
    where T: AsRef<Path>,
{
    let path = path.as_ref();
    if !path.is_file() {
        Ok(false)
    } else if path.extension().and_then(OsStr::to_str).eq(&Some("sketch")) {
        Ok(true)
    } else {
        Err(SkittyError::NotASketchFile(path.to_owned()))
    }
}

fn make_sketch<'a>(path: &'a Path) -> SkittyResult<Cow<'a, Path>>
{
//    let path = path.as_ref();
    if is_sketch_file(path)? {
        Ok(path.into())
    } else if path.is_dir() {
        Ok(path.with_extension("sketch").into())
    } else {
        Err(SkittyError::ProjectNotFound(path.to_owned()))
    }
}

fn get_dir<'a>(path: &'a Path) -> SkittyResult<Cow<'a, Path>>
{
    if path.is_dir() {
        return Ok(path.into());
    }
    let file = path.file_stem().ok_or(SkittyError::UnknownDirProblem(path.to_owned()))?;
    let dir = path.parent().ok_or(SkittyError::UnknownDirProblem(path.to_owned()))?;
    Ok(dir.join(file).into())

}

pub fn watch(path: &AsRef<Path>) -> SkittyResult<()>
{
    let absolute_path = make_absolute(path)?;
    let absolute_sketch_file = make_sketch(absolute_path.as_ref())?;
    let absolute_dir = get_dir(absolute_sketch_file.as_ref())?;


    let file_mtime = metadata(&absolute_sketch_file)?.modified()?;
    let dir_mtime = metadata(&absolute_dir)?.modified()?;

    println!("{:?}", dir_mtime > file_mtime);

    Ok(())
    // Create a channel to receive the events.
//    let (tx, rx) = channel();
//
//    // Automatically select the best implementation for your platform.
//    // You can also access each implementation directly e.g. INotifyWatcher.
//    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;
//
//    // Add a path to be watched. All files and directories at that path and
//    // below will be monitored for changes.
//    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
//
//    // This is a simple loop, but you may want to use more complex logic here,
//    // for example to handle I/O.
//    loop {
//        match rx.recv() {
//            Ok(event) => println!("{:?}", event),
//            Err(e) => println!("watch error: {:?}", e),
//        }
//    }
}
