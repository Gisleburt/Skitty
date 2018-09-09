use std::{
    borrow::Cow,
    env::current_dir,
    ffi::OsStr,
    fs::{read_dir, metadata},
    path::{Path, PathBuf},
};

use error::{SkittyError, SkittyResult};

pub fn newest_file_in_dir(dir: &AsRef<Path>) -> SkittyResult<Option<PathBuf>>
{
    let mut oldest_entry = None;
    if dir.as_ref().is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            if entry.path().is_dir() {
                newest_file_in_dir(&entry.path())?;
            }
            if entry.path().is_file() {
                if let Some(old_file) = oldest_entry {
                    let old_file_time = metadata(&old_file)?.modified()?;
                    let new_file_time = metadata(entry.path())?.modified()?;
                    if new_file_time < old_file_time {
                        oldest_entry = Some(entry.path())
                    } else {
                        oldest_entry = Some(old_file)
                    }
                } else {
                    oldest_entry = Some(entry.path());
                }
            }
        }
    }
    Ok(oldest_entry)
}

pub fn make_absolute<'a>(path: &'a AsRef<Path>) -> SkittyResult<Cow<'a, Path>>
{
    let path = path.as_ref();
    if path.is_absolute() {
        Ok(path.into())
    } else {
        Ok(current_dir()?.join(path).into())
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

pub fn make_sketch<'a>(path: &'a Path) -> SkittyResult<Cow<'a, Path>>
{
    if is_sketch_file(path)? {
        Ok(path.into())
    } else if path.is_dir() {
        Ok(path.with_extension("sketch").into())
    } else {
        Err(SkittyError::ProjectNotFound(path.to_owned()))
    }
}

pub fn get_dir<'a>(path: &'a Path) -> SkittyResult<Cow<'a, Path>>
{
    if path.is_dir() {
        return Ok(path.into());
    }
    let file = path.file_stem().ok_or(SkittyError::UnknownDirProblem(path.to_owned()))?;
    let dir = path.parent().ok_or(SkittyError::UnknownDirProblem(path.to_owned()))?;
    Ok(dir.join(file).into())
}
