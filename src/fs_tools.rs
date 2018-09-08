use std::{
    borrow::Cow,
    env::current_dir,
    path::{Path, PathBuf},
    ffi::OsStr,
    fs::metadata,
};

use error::{SkittyError, SkittyResult};

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

fn make_absolute<'a>(path: &'a AsRef<Path>) -> SkittyResult<Cow<'a, Path>>
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
