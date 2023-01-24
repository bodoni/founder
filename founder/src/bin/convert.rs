extern crate arguments;
extern crate founder;
extern crate resvg;

use std::io::Result;
use std::path::{Path, PathBuf};

fn main() {
    let arguments = arguments::parse(std::env::args()).unwrap();
    let path: PathBuf = match arguments.get::<String>("path") {
        Some(path) => path.into(),
        _ => {
            eprintln!("Error: --path should be given.");
            return;
        }
    };
    founder::scanning::scan_summarize(
        &path,
        &["svg"],
        process,
        arguments.get::<u32>("document-size").unwrap_or(28),
        arguments.get::<usize>("workers").unwrap_or(1),
        &vec![],
    );
}

fn process(path: &Path, document_size: u32) -> Result<Option<()>> {
    match to_png(path, document_size) {
        Ok(_) => {
            eprintln!("[success] {:?}", path);
            Ok(Some(()))
        }
        Err(error) => {
            eprintln!("[failure] {:?} ({:?})", path, error);
            Err(error)
        }
    }
}

fn to_png(path: &Path, document_size: u32) -> Result<()> {
    use std::io::{Error, ErrorKind};

    macro_rules! raise(
        () => (return Err(Error::new(ErrorKind::Other, "failed to convert to png")))
    );

    let data = std::fs::read(path)?;
    let options = resvg::usvg::Options::default();
    let tree = match resvg::usvg::Tree::from_data(&data, &options) {
        Ok(tree) => tree,
        _ => raise!(),
    };
    let mut map = match resvg::tiny_skia::Pixmap::new(document_size, document_size) {
        Some(map) => map,
        _ => raise!(),
    };
    let mut paint = resvg::tiny_skia::Paint::default();
    paint.set_color_rgba8(255, 255, 255, 255);
    map.fill_rect(
        resvg::tiny_skia::Rect::from_xywh(0.0, 0.0, document_size as f32, document_size as f32)
            .unwrap(),
        &paint,
        resvg::tiny_skia::Transform::identity(),
        None,
    )
    .unwrap();
    match resvg::render(
        &tree,
        resvg::usvg::FitTo::Size(document_size, document_size),
        resvg::tiny_skia::Transform::default(),
        map.as_mut(),
    ) {
        Some(_) => {}
        _ => raise!(),
    }
    match map.save_png(path.with_extension("png")) {
        Ok(_) => Ok(()),
        _ => raise!(),
    }
}
