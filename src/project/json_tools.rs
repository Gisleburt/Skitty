use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{File, read_dir},
    path::Path,
};
use serde_json::{self, Value};
use std::io::prelude::*;

use error::SkittyResult;

pub fn prettify_json_in_dir(dir: &AsRef<Path>) -> SkittyResult<()> {
    if dir.as_ref().is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                prettify_json_in_dir(&path)?;
            }
            if path.is_file() {
                if path.extension().and_then(OsStr::to_str).eq(&Some("json")) {
                    prettify_json_file(&path)?;
                }
            }
        }
    }
    Ok(())
}

fn prettify_json_file(file: &AsRef<Path>) -> SkittyResult<()> {
    println!("Prettifying JSON {}", file.as_ref().to_string_lossy());
    let json_file = File::open(&file)?;
    let json: HashMap<String, Value> = serde_json::from_reader(&json_file)?;
    let pretty_json = serde_json::to_string_pretty(&json)?;

    let mut json_file = File::create(&file)?;
    json_file.write(pretty_json.as_bytes())?;
    Ok(())
}
