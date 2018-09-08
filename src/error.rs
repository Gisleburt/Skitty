use std::io::Error as IoError;
use notify::Error as NotifyError;
use zip::result::ZipError;
use std::result::Result;
use std::path::PathBuf;

pub type SkittyResult<T> = Result<T, SkittyError>;

#[derive(Debug, Fail)]
pub enum SkittyError {
    #[fail(display = "Notify error: {}", _0)]
    NotifyError(NotifyError),
    #[fail(display = "Zip error: {}", _0)]
    ZipError(ZipError),
    #[fail(display = "IO Error: {}", _0)]
    IoError(IoError),
    #[fail(display = "Project not found: {:?}", _0)]
    ProjectNotFound(PathBuf),
    #[fail(display = "{:?} is not a sketch file", _0)]
    NotASketchFile(PathBuf),
    #[fail(display = "Something went wrong getting the dir name from the file name {:?}", _0)]
    UnknownDirProblem(PathBuf),
}

impl From<NotifyError> for SkittyError {
    fn from(err: NotifyError) -> SkittyError {
        SkittyError::NotifyError(err)
    }
}

impl From<ZipError> for SkittyError {
    fn from(err: ZipError) -> SkittyError {
        SkittyError::ZipError(err)
    }
}

impl From<IoError> for SkittyError {
    fn from(err: IoError) -> SkittyError {
        SkittyError::IoError(err)
    }
}
