extern crate arguments;
extern crate font;
extern crate walkdir;

use font::Font;
use std::io;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
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
    let ignores = arguments.get_all::<String>("ignore").unwrap_or(vec![]);
    let workers = arguments.get::<usize>("workers").unwrap_or(1);

    let (forward_sender, forward_receiver) = mpsc::channel::<PathBuf>();
    let (backward_sender, backward_receiver) = mpsc::channel::<(PathBuf, io::Result<()>)>();
    let forward_receiver = Arc::new(Mutex::new(forward_receiver));

    let _: Vec<_> = (0..workers)
        .map(|_| {
            let forward_receiver = forward_receiver.clone();
            let backward_sender = backward_sender.clone();
            thread::spawn(move || loop {
                let path = forward_receiver.lock().unwrap().recv().unwrap();
                backward_sender.send(process(path)).unwrap();
            })
        })
        .collect();
    let mut count = 0;
    for entry in WalkDir::new(&path)
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
    {
        forward_sender.send(entry.path().into()).unwrap();
        count += 1;
    }
    let values: Vec<(PathBuf, io::Result<()>)> = (0..count)
        .map(|_| backward_receiver.recv().unwrap())
        .collect();
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
