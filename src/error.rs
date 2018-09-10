use notify::Error as NotifyError;
use serde_json::Error as JsonError;
use zip::result::ZipError;
use std::{
    io::Error as IoError,
    result::Result,
    path::{PathBuf, StripPrefixError},
    sync::mpsc::RecvError as ChannelReceiveError,
    time::SystemTimeError,
};

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
    #[fail(display = "{:?} is not a sketch file or project directory", _0)]
    NotASketchFile(PathBuf),
    #[fail(display = "Something went wrong getting the dir name from the file name: {:?}", _0)]
    UnknownDirProblem(PathBuf),
    #[fail(display = "Unable to make path relative while zipping: {:?}", _0)]
    StripPrefixError(StripPrefixError),
    #[fail(display = "Problem reading file systemL {:?}", _0)]
    FileSystemUnreadable(PathBuf),
    #[fail(display = "Channel was broken, can not receive messages {:?}", _0)]
    ChannelReceiveError(ChannelReceiveError),
    #[fail(display = "Something went wrong comparing file times: {:?}", _0)]
    SystemTimeError(SystemTimeError),
    #[fail(display = "Unable to read JSON: {:?}", _0)]
    JsonError(JsonError),
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

impl From<StripPrefixError> for SkittyError {
    fn from(err: StripPrefixError) -> SkittyError {
        SkittyError::StripPrefixError(err)
    }
}

impl From<ChannelReceiveError> for SkittyError {
    fn from(err: ChannelReceiveError) -> SkittyError {
        SkittyError::ChannelReceiveError(err)
    }
}

impl From<SystemTimeError> for SkittyError {
    fn from(err: SystemTimeError) -> SkittyError {
        SkittyError::SystemTimeError(err)
    }
}

impl From<JsonError> for SkittyError {
    fn from(err: JsonError) -> SkittyError {
        SkittyError::JsonError(err)
    }
}
