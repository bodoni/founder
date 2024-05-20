mod support;

use std::io::Result;
use std::path::{Path, PathBuf};

use colored::Colorize;
use resvg::tiny_skia::{Paint, Pixmap, Rect, Transform};
use resvg::usvg::Tree;

fn main() {
    let arguments = arguments::parse(std::env::args()).unwrap();
    let path: PathBuf = arguments
        .get::<String>("path")
        .unwrap_or_else(|| ".".to_string())
        .into();
    let excludes = arguments.get_all::<String>("exclude").unwrap_or_default();
    let excludes = excludes.iter().map(String::as_str).collect::<Vec<_>>();
    support::summarize(
        &folder::scan(
            &path,
            |path| support::filter(path, &[".svg"], &excludes),
            process,
            arguments.get::<u32>("document-size").unwrap_or(28),
            arguments.get::<usize>("workers"),
        )
        .collect::<Vec<_>>(),
    );
}

fn process(path: &Path, document_size: u32) -> Result<Option<()>> {
    match convert(path, document_size) {
        Ok(_) => {
            eprintln!("{} {path:?}", "[success]".green());
            Ok(Some(()))
        }
        Err(error) => {
            eprintln!("{} {path:?} ({error:?})", "[failure]".red());
            Err(error)
        }
    }
}

fn convert(path: &Path, document_size: u32) -> Result<()> {
    macro_rules! raise(
        () => (return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "failed to convert to png",
        )))
    );

    let mut canvas = match Pixmap::new(document_size, document_size) {
        Some(value) => value,
        _ => raise!(),
    };
    let content = match Tree::from_data(&std::fs::read(path)?, &Default::default()) {
        Ok(value) => value,
        _ => raise!(),
    };
    let mut paint = Paint::default();
    paint.set_color_rgba8(255, 255, 255, 255);
    canvas.fill_rect(
        Rect::from_xywh(0.0, 0.0, document_size as f32, document_size as f32).unwrap(),
        &paint,
        Transform::identity(),
        None,
    );
    resvg::render(&content, Transform::default(), &mut canvas.as_mut());
    match canvas.save_png(path.with_extension("png")) {
        Ok(_) => Ok(()),
        _ => raise!(),
    }
}
