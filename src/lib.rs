#[macro_use]
extern crate failure;
extern crate notify;
extern crate zip;

mod error;

use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use zip::{read::ZipFile, ZipArchive, ZipWriter};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::path::{Path, PathBuf};
use std::fs::{File, read_dir, create_dir_all, metadata};
use std::io::{copy, prelude::*};
use error::{SkittyError, SkittyResult};
use std::borrow::Cow;
use std::ffi::OsStr;
use std::time::UNIX_EPOCH;

fn zipfile_to_file<T>(mut zipfile: ZipFile, dir: T)
    where T: AsRef<Path>,
{
    // Test Path is acceptable
    let file_path = dir.as_ref().join(zipfile.sanitized_name());

    if !file_path.to_string_lossy().contains(dir.as_ref().to_string_lossy().as_ref()) {
        panic!("zip contained path traversal");
    }

    let _ = create_dir_all(file_path.parent().expect("Could not open dir"));

    let mut file = File::create(&file_path).expect("Could not create file");
    let r = copy(&mut zipfile, &mut file);
    println!("{}", file_path.to_string_lossy());
}

pub fn zip_to_dir<T, U>(from: T, to: U) -> SkittyResult<()>
    where T: AsRef<Path>,
          U: AsRef<Path>,
{
    create_dir_all(to.as_ref());
    let file = File::open(from.as_ref()).expect("Couldn't open file");
    let mut zip = ZipArchive::new(file)?;
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        zipfile_to_file(file, to.as_ref());
    }
    Ok(())
}

pub fn dir_to_zip<T, U>(from: T, to: U) -> SkittyResult<()>
    where T: AsRef<Path>,
          U: AsRef<Path>,
{
    let mut file = File::create(to.as_ref()).expect("Couldn't open file");
    let mut zip = zip::ZipWriter::new(file);

    dir_to_zip_recurse(&mut zip, from.as_ref(), from.as_ref());

    Ok(())
}

fn dir_to_zip_recurse<T, U>(zip: &mut ZipWriter<File>, dir: T, root: U)
    where T: AsRef<Path>,
          U: AsRef<Path>,
{
    if dir.as_ref().is_dir() {
        for entry in read_dir(dir).expect("Couldn't read dir") {
            let entry = entry.expect("Something wrong with the entry");
            if entry.path().is_dir() {
                dir_to_zip_recurse(zip, entry.path(), root.as_ref());
            }
            if entry.path().is_file() {
                write_file_to_zip(zip, entry.path(), root.as_ref());
            }
        }
    }
}

fn write_file_to_zip<T, U>(zip: &mut ZipWriter<File>, file_path: T, root_path: U) -> SkittyResult<()>
    where T: AsRef<Path>,
          U: AsRef<Path>,
{
    let relative_path = file_path.as_ref().strip_prefix(root_path.as_ref()).expect("Paths not related");

    let mut bytes: Vec<u8> = Vec::new();
    let mut file = File::open(&file_path).expect("Couldn't open file");
    file.read_to_end(&mut bytes);

    let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
    zip.start_file(relative_path.to_string_lossy(), options)?;
    zip.write(&bytes)?;
    Ok(())
}

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
