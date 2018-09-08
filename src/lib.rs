#[macro_use]
extern crate failure;
extern crate notify;
extern crate zip;

mod error;
mod project;

use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use std::sync::mpsc::channel;
use std::path::Path;
use error::SkittyResult;
use project::Project;


pub fn watch(path: &AsRef<Path>) -> SkittyResult<()>
{
    let project = Project::from(path)?;
    println!("{:?}", project.is_git_newer()?);

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
