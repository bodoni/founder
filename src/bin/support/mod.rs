use std::io::Result;
use std::path::{Path, PathBuf};

pub fn filter(path: &Path, includes: &[&str], excludes: &[&str]) -> bool {
    let path = path.to_str().unwrap();
    includes.iter().any(|value| path.ends_with(value))
        && !excludes.iter().any(|value| path.contains(value))
}

pub fn summarize<T>(values: &[(PathBuf, Result<Option<T>>)])
where
    T: Send + 'static,
{
    let (success, failure): (Vec<_>, Vec<_>) =
        values.iter().partition(|(_, result)| result.is_ok());
    let (complete, incomplete): (Vec<_>, Vec<_>) = success
        .into_iter()
        .partition(|(_, result)| result.as_ref().unwrap().is_some());
    eprintln!("Found {} complete.", complete.len());
    eprintln!("Found {} incomplete.", incomplete.len());
    for (path, _) in incomplete.iter() {
        eprintln!("{path:?}");
    }
    eprintln!("Found {} failed.", failure.len());
    for (path, result) in failure.iter() {
        eprintln!("{:?}: {}", path, result.as_ref().err().unwrap());
    }
    std::process::exit(i32::from(!failure.is_empty()));
}
