use std::io;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

use font::Glyph;
use svg::node::element::Group;
use walkdir::WalkDir;

#[allow(dead_code)]
pub fn draw(glyph: &Glyph) -> Group {
    use font::{Offset, Segment};
    use svg::node::element::path::Data;
    use svg::node::element::Path;
    use svg::node::Node;

    let mut group = Group::new();
    let mut a = Offset::default();
    for contour in glyph.iter() {
        a += contour.offset;
        let mut data = Data::new().move_to(vec![a.0, a.1]);
        for segment in contour.iter() {
            match segment {
                &Segment::Linear(b) => {
                    a += b;
                    data = data.line_by(vec![b.0, b.1]);
                }
                &Segment::Cubic(b, mut c, mut d) => {
                    c += b;
                    d += c;
                    a += d;
                    data = data.cubic_curve_by(vec![b.0, b.1, c.0, c.1, d.0, d.1]);
                }
                _ => unreachable!(),
            }
        }
        data = data.close();
        group.append(Path::new().set("d", data));
    }
    group
}

#[allow(dead_code)]
pub fn scan<F>(path: &Path, process: F, workers: usize) -> Vec<(PathBuf, io::Result<()>)>
where
    F: Fn(PathBuf) -> (PathBuf, io::Result<()>) + Copy + Send + 'static,
{
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
