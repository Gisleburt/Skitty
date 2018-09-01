extern crate skitty;

use skitty::{watch, zip_to_dir};

fn main() {
    let path = std::env::current_dir().expect("current dir");
    let path = path.to_str().expect("to str");

    let zip = "/Users/danielmason/projects/rust/skitty/example/HelloWorld.sketch";
    let dir = "/Users/danielmason/projects/rust/skitty/example/HelloWorld";
    if let Err(e) = zip_to_dir(zip, dir) {
        println!("error: {:?}", e)
    }
}
