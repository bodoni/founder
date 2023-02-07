use std::io::Result;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

use walkdir::WalkDir;

pub fn scan<F1, F2, T, U>(
    path: &Path,
    filter: F1,
    process: F2,
    parameter: T,
    workers: usize,
) -> Vec<(PathBuf, Result<U>)>
where
    F1: Fn(&Path) -> bool,
    F2: Fn(&Path, T) -> Result<U> + Copy + Send + 'static,
    T: Clone + Send + 'static,
    U: Send + 'static,
{
    let (forward_sender, forward_receiver) = mpsc::channel::<PathBuf>();
    let (backward_sender, backward_receiver) = mpsc::channel::<(PathBuf, Result<U>)>();
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
                    .send(wrap(path, process, parameter.clone()))
                    .unwrap();
            })
        })
        .collect();
    let mut count = 0;
    for entry in WalkDir::new(path)
        .into_iter()
        .map(|entry| entry.unwrap())
        .filter(|entry| !entry.file_type().is_dir())
        .filter(|entry| filter(entry.path()))
    {
        forward_sender.send(entry.path().into()).unwrap();
        count += 1;
    }
    (0..count)
        .map(|_| backward_receiver.recv().unwrap())
        .collect()
}

pub fn scan_summarize<F1, F2, T, U>(
    path: &Path,
    filter: F1,
    process: F2,
    parameter: T,
    workers: usize,
    ignores: &[String],
) where
    F1: Fn(&Path) -> bool,
    F2: Fn(&Path, T) -> Result<Option<U>> + Copy + Send + 'static,
    T: Clone + Send + 'static,
    U: Send + 'static,
{
    let values = scan(path, filter, process, parameter, workers);
    let (positives, negatives): (Vec<_>, Vec<_>) =
        values.into_iter().partition(|(_, result)| result.is_ok());
    let (complete, incomplete): (Vec<_>, Vec<_>) = positives
        .into_iter()
        .partition(|(_, result)| result.as_ref().unwrap().is_some());
    let (ignored, failed): (Vec<_>, Vec<_>) = negatives.into_iter().partition(|(path, _)| {
        let path = path.to_str().unwrap();
        ignores.iter().any(|name| path.contains(name))
    });
    eprintln!("Complete: {}", complete.len());
    eprintln!("Incomplete: {}", incomplete.len());
    for (path, _) in incomplete.iter() {
        eprintln!("{path:?}");
    }
    eprintln!("Ignored: {}", ignored.len());
    for (path, result) in ignored.iter() {
        eprintln!("{:?}: {}", path, result.as_ref().err().unwrap());
    }
    eprintln!("Failed: {}", failed.len());
    for (path, result) in failed.iter() {
        eprintln!("{:?}: {}", path, result.as_ref().err().unwrap());
    }
    assert_eq!(failed.len(), 0);
}

fn wrap<F, T, U>(path: PathBuf, process: F, parameter: T) -> (PathBuf, Result<U>)
where
    F: Fn(&Path, T) -> Result<U> + Copy + Send + 'static,
    T: Clone + Send + 'static,
    U: Send + 'static,
{
    let result = process(&path, parameter);
    (path, result)
}
