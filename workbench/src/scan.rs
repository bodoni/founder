extern crate arguments;
extern crate font;
extern crate futures;

use font::Font;
use futures::channel::mpsc;
use futures::executor;
use futures::executor::ThreadPool;
use futures::StreamExt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() {
    let arguments = arguments::parse(std::env::args()).unwrap();
    let path: PathBuf = match arguments.get::<String>("path") {
        Some(path) => path.into(),
        _ => {
            println!("Error: --path should be given.");
            return;
        }
    };

    let pool = ThreadPool::new().unwrap();
    let (tx, mut rx) = mpsc::unbounded::<PathBuf>();
    let futures = async {
        let result = async move {
            list(&path, |path| Ok(tx.unbounded_send(path.into()).unwrap())).unwrap();
        };
        pool.spawn_ok(result);
        let mut pending = vec![];
        while let Some(path) = rx.next().await {
            let result = process(&path);
            pending.push((path, result));
        }
        pending
    };
    let values: Vec<(PathBuf, io::Result<()>)> = executor::block_on(futures);
    let (successes, failures): (Vec<_>, Vec<_>) =
        values.into_iter().partition(|(_, result)| result.is_ok());
    println!("Successes: {}", successes.len());
    println!("Failures: {}", failures.len());
    for (path, result) in failures.iter() {
        println!("{:?}: {:?}", path, result);
    }
    assert_eq!(failures.len(), 0);
}

fn list<F>(path: &Path, callback: F) -> io::Result<()>
where
    F: Fn(&Path) -> io::Result<()> + Copy,
{
    if !path.is_dir() {
        return Ok(());
    }
    for entry in fs::read_dir(path)? {
        let path = entry?.path();
        if path.is_dir() {
            list(&path, callback)?;
            continue;
        }
        match path.extension().and_then(|extension| extension.to_str()) {
            Some("otf") => callback(&path)?,
            Some("ttf") => callback(&path)?,
            _ => {}
        }
    }
    Ok(())
}

fn process(path: &Path) -> io::Result<()> {
    println!("Processing {:?}...", path);
    Font::open(path)?;
    Ok(())
}
