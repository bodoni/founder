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

fn wrap<F, T, U>(path: PathBuf, process: F, parameter: T) -> (PathBuf, Result<U>)
where
    F: Fn(&Path, T) -> Result<U> + Copy + Send + 'static,
    T: Clone + Send + 'static,
    U: Send + 'static,
{
    let result = process(&path, parameter);
    (path, result)
}
