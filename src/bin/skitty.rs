#[macro_use]
extern crate clap;
#[macro_use]
extern crate failure;

extern crate skitty;

use clap::App;

use skitty::watch;

fn main() {
    let yaml = load_yaml!("skitty.yaml");
    let mut app = App::from_yaml(yaml)
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"));
    let matches = app.clone().get_matches();

    match matches.subcommand() {
        ("watch", Some(matches)) => {
            if let Some(file) = matches.value_of("FILE") {
                if let Err(e) = watch(&file) {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            } else {
                eprintln!("You must specify a file or directory to watch");
                std::process::exit(1);
            }
        },
        _ => {
            app.print_help();
        },
    }


//    let path = std::env::current_dir().expect("current dir");
//    let path = path.to_str().expect("to str");
//
//    let zip = "/Users/danielmason/projects/rust/skitty/example/hello-world.sketch";
//    let dir = "/Users/danielmason/projects/rust/skitty/example/hell0-world";
//    if let Err(e) = dir_to_zip(dir, zip) {
//        println!("error: {:?}", e)
//    }
}
