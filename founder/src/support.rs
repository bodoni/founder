use std::io::Result;
use std::path::PathBuf;

pub fn summarize<T>(values: &[(PathBuf, Result<Option<T>>)], ignores: &[String])
where
    T: Send + 'static,
{
    let (positives, negatives): (Vec<_>, Vec<_>) =
        values.iter().partition(|(_, result)| result.is_ok());
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
