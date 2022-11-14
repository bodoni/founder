extern crate arguments;
extern crate font;
extern crate futures;
extern crate walkdir;

use font::Font;
use futures::executor::block_on;
use futures::future::join_all;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() {
    let arguments = arguments::parse(std::env::args()).unwrap();
    let path: PathBuf = match arguments.get::<String>("path") {
        Some(path) => path.into(),
        _ => {
            println!("Error: --path should be given.");
            return;
        }
    };
    let mut futures = vec![];
    for entry in WalkDir::new(&path).into_iter().map(|entry| entry.unwrap()) {
        if entry.file_type().is_dir() {
            continue;
        }
        match entry
            .path()
            .extension()
            .and_then(|extension| extension.to_str())
        {
            Some("otf") | Some("ttf") => {}
            _ => continue,
        }
        println!("Registering {:?}...", entry.path());
        futures.push(register(entry.path().into()));
    }
    let values = block_on(join_all(futures));
    let (successes, failures): (Vec<_>, Vec<_>) =
        values.into_iter().partition(|(_, result)| result.is_ok());
    println!("Successes: {}", successes.len());
    println!("Failures: {}", failures.len());
    for (path, result) in failures.iter() {
        println!("{:?}: {:?}", path, result);
    }
    assert_eq!(failures.len(), 0);
}

fn process(path: &Path) -> io::Result<()> {
    println!("Processing {:?}...", path);
    Font::open(path)?;
    Ok(())
}

async fn register(path: PathBuf) -> (PathBuf, io::Result<()>) {
    let result = process(&path);
    (path, result)
}
