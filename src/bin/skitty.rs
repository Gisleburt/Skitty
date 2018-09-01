extern crate skitty;

use skitty::watch;

fn main() {
    let path = std::env::current_dir().expect("current dir");
    let path = path.to_str().expect("to str");
    if let Err(e) = watch(path) {
        println!("error: {:?}", e)
    }
}
