extern crate arguments;
extern crate font;
extern crate founder;
extern crate walkdir;

use std::io::Result;
use std::path::{Path, PathBuf};

use font::File;

fn main() {
    let arguments = arguments::parse(std::env::args()).unwrap();
    let path: PathBuf = match arguments.get::<String>("path") {
        Some(path) => path.into(),
        _ => {
            eprintln!("Error: --path should be given.");
            return;
        }
    };
    founder::scanning::scan_summarize(
        &path,
        process,
        (),
        arguments.get::<usize>("workers").unwrap_or(1),
        &arguments.get_all::<String>("ignore").unwrap_or(vec![]),
    );
}

fn process(path: &Path, _: ()) -> Result<Option<()>> {
    match File::open(&path) {
        Ok(_) => {
            eprintln!("[success] {:?}", path);
            Ok(Some(()))
        }
        Err(error) => {
            eprintln!("[failure] {:?} ({:?})", path, error);
            Err(error)
        }
    }
}
