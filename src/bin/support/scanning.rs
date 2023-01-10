use std::io;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

use walkdir::WalkDir;

#[allow(dead_code)]
pub fn scan<F, T, U>(
    path: &Path,
    process: F,
    parameter: T,
    workers: usize,
) -> Vec<(PathBuf, io::Result<U>)>
where
    F: Fn(PathBuf, T) -> (PathBuf, io::Result<U>) + Copy + Send + 'static,
    T: Clone + Send + 'static,
    U: Send + 'static,
{
    let (forward_sender, forward_receiver) = mpsc::channel::<PathBuf>();
    let (backward_sender, backward_receiver) = mpsc::channel::<(PathBuf, io::Result<U>)>();
    let forward_receiver = Arc::new(Mutex::new(forward_receiver));

    let _: Vec<_> = (0..workers)
        .map(|_| {
            let forward_receiver = forward_receiver.clone();
            let backward_sender = backward_sender.clone();
            let parameter = parameter.clone();
            thread::spawn(move || loop {
                let path = match forward_receiver.lock().unwrap().recv() {
                    Ok(path) => path,
                    Err(_) => break,
                };
                backward_sender
                    .send(process(path, parameter.clone()))
                    .unwrap();
            })
        })
        .collect();
    let mut count = 0;
    for entry in WalkDir::new(path)
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
    return (0..count)
        .map(|_| backward_receiver.recv().unwrap())
        .collect();
}

#[allow(dead_code)]
pub fn scan_summarize<F, T, U>(
    path: &Path,
    process: F,
    parameter: T,
    workers: usize,
    ignores: &[String],
) where
    F: Fn(PathBuf, T) -> (PathBuf, io::Result<Option<U>>) + Copy + Send + 'static,
    T: Clone + Send + 'static,
    U: Send + 'static,
{
    let values = scan(path, process, parameter, workers);
    let (positives, negatives): (Vec<_>, Vec<_>) =
        values.into_iter().partition(|(_, result)| result.is_ok());
    let (complete, incomplete): (Vec<_>, Vec<_>) = positives
        .into_iter()
        .partition(|(_, result)| result.as_ref().unwrap().is_some());
    let (ignored, failed): (Vec<_>, Vec<_>) = negatives.into_iter().partition(|(path, _)| {
        let path = path.to_str().unwrap();
        ignores.iter().any(|name| path.contains(name))
    });
    println!("Complete: {}", complete.len());
    println!("Incomplete: {}", incomplete.len());
    for (path, _) in incomplete.iter() {
        println!("{:?}", path);
    }
    println!("Ignored: {}", ignored.len());
    for (path, result) in ignored.iter() {
        println!("{:?}: {}", path, result.as_ref().err().unwrap());
    }
    println!("Failed: {}", failed.len());
    for (path, result) in failed.iter() {
        println!("{:?}: {}", path, result.as_ref().err().unwrap());
    }
    assert_eq!(failed.len(), 0);
}
