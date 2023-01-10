extern crate arguments;
extern crate font;
extern crate walkdir;

mod support;

use std::io::Result;
use std::path::PathBuf;

use font::File;

fn main() {
    let arguments = arguments::parse(std::env::args()).unwrap();
    let path: PathBuf = match arguments.get::<String>("path") {
        Some(path) => path.into(),
        _ => {
            println!("Error: --path should be given.");
            return;
        }
    };
    support::scanning::scan_summarize(
        &path,
        process,
        (),
        arguments.get::<usize>("workers").unwrap_or(1),
        &arguments.get_all::<String>("ignore").unwrap_or(vec![]),
    );
}

fn process(path: PathBuf, _: ()) -> (PathBuf, Result<Option<()>>) {
    let result = match File::open(&path) {
        Ok(_) => {
            println!("[success] {:?}", path);
            Ok(Some(()))
        }
        Err(error) => {
            println!("[failure] {:?} ({:?})", path, error);
            Err(error)
        }
    };
    (path, result)
}
