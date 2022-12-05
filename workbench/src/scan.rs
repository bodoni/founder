extern crate arguments;
extern crate font;
extern crate walkdir;

use font::Font;
use std::io;
use std::path::PathBuf;

mod support;

fn main() {
    let arguments = arguments::parse(std::env::args()).unwrap();
    let path: PathBuf = match arguments.get::<String>("path") {
        Some(path) => path.into(),
        _ => {
            println!("Error: --path should be given.");
            return;
        }
    };
    let ignores = arguments.get_all::<String>("ignore").unwrap_or(vec![]);
    let workers = arguments.get::<usize>("workers").unwrap_or(1);

    let values = support::process(&path, process, workers);
    let (successes, other): (Vec<_>, Vec<_>) =
        values.into_iter().partition(|(_, result)| result.is_ok());
    let (ignores, failures): (Vec<_>, Vec<_>) = other.into_iter().partition(|(path, _)| {
        let path = path.to_str().unwrap();
        ignores.iter().any(|name| path.contains(name))
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

fn process(path: PathBuf) -> (PathBuf, io::Result<()>) {
    let result = match Font::open(&path) {
        Ok(_) => {
            println!("[success] {:?}", path);
            Ok(())
        }
        Err(error) => {
            println!("[failure] {:?} ({:?})", path, error);
            Err(error)
        }
    };
    (path, result)
}
