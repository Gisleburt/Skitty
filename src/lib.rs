extern crate notify;
extern crate zip;

use notify::{RecommendedWatcher, Watcher, RecursiveMode};
use zip::{read::ZipFile, ZipArchive, ZipWriter, result::ZipResult};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::path::Path;
use std::fs::{File, read_dir, create_dir_all};
use std::io::{copy, prelude::*};

fn zipfile_to_file<T>(mut zipfile: ZipFile, dir: T)
    where T: AsRef<Path>
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

pub fn zip_to_dir<T, U>(from: T, to: U) -> ZipResult<()>
    where T: AsRef<Path>,
          U: AsRef<Path>
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

pub fn dir_to_zip<T, U>(from: T, to: U) -> zip::result::ZipResult<()>
    where T: AsRef<Path>,
          U: AsRef<Path>
{
    let mut file = File::create(to.as_ref()).expect("Couldn't open file");
    let mut zip = zip::ZipWriter::new(file);

    dir_to_zip_recurse(&mut zip, from.as_ref(), from.as_ref());

    Ok(())
}

fn dir_to_zip_recurse<T, U>(zip: &mut ZipWriter<File>, dir: T, root: U)
    where T: AsRef<Path>,
          U: AsRef<Path>
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

fn write_file_to_zip<T, U>(zip: &mut ZipWriter<File>, file_path: T, root_path: U) -> ZipResult<()>
    where T: AsRef<Path>,
          U: AsRef<Path>
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

pub fn watch<T>(path: T) -> notify::Result<()>
    where T: AsRef<str>
{
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    loop {
        match rx.recv() {
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
