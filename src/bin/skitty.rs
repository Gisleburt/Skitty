extern crate skitty;

use skitty::{watch, dir_to_zip};

fn main() {
    let path = std::env::current_dir().expect("current dir");
    let path = path.to_str().expect("to str");

    let zip = "/Users/danielmason/projects/rust/skitty/example/HelloWorld2.sketch";
    let dir = "/Users/danielmason/projects/rust/skitty/example/HelloWorld";
    if let Err(e) = dir_to_zip(dir, zip) {
        println!("error: {:?}", e)
    }
}
