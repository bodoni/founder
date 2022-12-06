extern crate arguments;
extern crate font;
extern crate svg;
#[macro_use(raise)]
extern crate typeface;
extern crate walkdir;

use std::io::Result;
use std::path::{Path, PathBuf};

use font::Font;
use svg::node::element::Style;
use svg::Document;

mod drawing;
mod scanning;

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
    let values = scanning::scan(&path, process, workers);
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

fn process(path: PathBuf) -> (PathBuf, Result<()>) {
    let result = match draw(&path) {
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

fn draw(path: &Path) -> Result<()> {
    let font = Font::open(path)?;
    let glyph = match font.draw('a')? {
        Some(glyph) => glyph,
        _ => raise!("failed to find the glyph"),
    };
    let (width, height) = (glyph.advance_width(), glyph.height() + 2.0 * 50.0);
    let transform = format!("translate(0, {}) scale(1, -1)", glyph.bounding_box.3 + 50.0);
    let glyph = drawing::draw(&glyph).set("transform", transform);
    let style = Style::new("path { fill: none; stroke: black; stroke-width: 3; }");
    let _ = Document::new()
        .set("width", width)
        .set("height", height)
        .add(style)
        .add(glyph);
    Ok(())
}
