use std::{
    fs::{File, create_dir_all, read_dir},
    io::{copy, prelude::*},
    path::Path,
};
use zip::{
    CompressionMethod::Stored,
    read::ZipFile,
    write::FileOptions,
    ZipArchive,
    ZipWriter,
};

use error::SkittyResult;

/// Extract a zip file to a given dir
fn zip_to_dir<T, U>(from: T, to: U) -> SkittyResult<()>
    where T: AsRef<Path>,
          U: AsRef<Path>,
{
    create_dir_all(to.as_ref())?;
    let file = File::open(from.as_ref()).expect("Couldn't open file");
    let mut zip = ZipArchive::new(file)?;
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        zipfile_to_file(file, to.as_ref())?;
    }
    Ok(())
}

/// Write a directory to a zip file
fn dir_to_zip<T, U>(from: T, to: U) -> SkittyResult<()>
    where T: AsRef<Path>,
          U: AsRef<Path>,
{
    let file = File::create(to.as_ref()).expect("Couldn't open file");
    let mut zip = ZipWriter::new(file);

    dir_to_zip_recurse(&mut zip, from.as_ref(), from.as_ref())?;

    Ok(())
}

/// Write a single file from the zip file to the given dir
fn zipfile_to_file<T>(mut zipfile: ZipFile, dir: T) -> SkittyResult<()>
    where T: AsRef<Path>,
{
    // Test Path is acceptable
    let file_path = dir.as_ref().join(zipfile.sanitized_name());
    if !file_path.to_string_lossy().contains(dir.as_ref().to_string_lossy().as_ref()) {
        panic!("zip contained path traversal");
    }

    create_dir_all(file_path.parent().expect("Could not open dir"))?;

    let mut file = File::create(&file_path).expect("Could not create file");
    copy(&mut zipfile, &mut file)?;
    println!("{}", file_path.to_string_lossy());
    Ok(())
}

/// Recursively traverse directories inside the given zip file and write them to the file system
fn dir_to_zip_recurse<T, U>(zip: &mut ZipWriter<File>, dir: T, root: U) -> SkittyResult<()>
    where T: AsRef<Path>,
          U: AsRef<Path>,
{
    if dir.as_ref().is_dir() {
        for entry in read_dir(dir).expect("Couldn't read dir") {
            let entry = entry.expect("Something wrong with the entry");
            if entry.path().is_dir() {
                dir_to_zip_recurse(zip, entry.path(), root.as_ref())?;
            }
            if entry.path().is_file() {
                write_file_to_zip(zip, entry.path(), root.as_ref())?;
            }
        }
    }
    Ok(())
}

/// Write a single file to the file system
fn write_file_to_zip<T, U>(zip: &mut ZipWriter<File>, file_path: T, root_path: U) -> SkittyResult<()>
    where T: AsRef<Path>,
          U: AsRef<Path>,
{
    let relative_path = file_path.as_ref().strip_prefix(root_path.as_ref())?;

    let mut bytes: Vec<u8> = Vec::new();
    let mut file = File::open(&file_path)?;
    file.read_to_end(&mut bytes)?;

    let options = FileOptions::default().compression_method(Stored);
    zip.start_file(relative_path.to_string_lossy(), options)?;
    zip.write(&bytes)?;
    Ok(())
}
