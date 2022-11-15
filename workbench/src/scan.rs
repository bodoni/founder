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
    let futures = WalkDir::new(&path)
        .into_iter()
        .map(|entry| entry.unwrap())
        .filter(|entry| !entry.file_type().is_dir())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .and_then(|extension| extension.to_str())
                .map(|extension| extension == "otf" || extension == "ttf")
                .unwrap_or(false)
        })
        .map(|entry| register(entry.path().into()));
    let values = block_on(join_all(futures));
    let (successes, other): (Vec<_>, Vec<_>) =
        values.into_iter().partition(|(_, result)| result.is_ok());
    let (ignores, failures): (Vec<_>, Vec<_>) = other.into_iter().partition(|(path, _)| {
        // https://github.com/google/fonts/issues/5551
        path.to_str().unwrap().contains("ubuntu")
    });
    println!("Successes: {}", successes.len());
    println!("Failures: {}", failures.len());
    for (path, result) in failures.iter() {
        println!("{:?}: {}", path, result.as_ref().err().unwrap());
    }
    println!("Ignores: {}", ignores.len());
    for (path, result) in ignores.iter() {
        println!("{:?}: {}", path, result.as_ref().err().unwrap());
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
