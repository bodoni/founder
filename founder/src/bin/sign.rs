extern crate arguments;
extern crate font;
extern crate founder;
extern crate svg;
extern crate walkdir;

use std::io::Result;
use std::path::{Path, PathBuf};

use font::File;
use svg::node::{element, Node};

fn main() {
    let arguments = arguments::parse(std::env::args()).unwrap();
    let path: PathBuf = match arguments.get::<String>("path") {
        Some(path) => path.into(),
        _ => {
            eprintln!("Error: --path should be given.");
            return;
        }
    };
    let characters: Vec<_> = match arguments.get::<String>("characters") {
        Some(characters) => characters.chars().collect(),
        _ => {
            eprintln!("Error: --characters should be given.");
            return;
        }
    };
    let output: Option<PathBuf> = match arguments.get::<String>("output") {
        Some(output) => Some(output.into()),
        _ => None,
    };
    founder::scanning::scan_summarize(
        &path,
        process,
        (characters, output),
        arguments.get::<usize>("workers").unwrap_or(1),
        &arguments.get_all::<String>("ignore").unwrap_or(vec![]),
    );
}

fn process(path: &Path, (characters, output): (Vec<char>, Option<PathBuf>)) -> Result<Option<()>> {
    const DOCUMENT_SIZE: f32 = 512.0;
    let group = match subprocess(&path, &characters, DOCUMENT_SIZE) {
        Ok(None) => {
            eprintln!("[missing] {:?}", path);
            return Ok(None);
        }
        Err(error) => {
            eprintln!("[failure] {:?} ({:?})", path, error);
            return Err(error);
        }
        Ok(Some(group)) => group,
    };
    let style = element::Style::new("path { fill: black; fill-rule: nonzero; }");
    let document = element::SVG::new()
        .set("width", DOCUMENT_SIZE)
        .set("height", DOCUMENT_SIZE)
        .add(style)
        .add(group);
    let output = match output {
        None => {
            eprintln!("[success] {:?}", path);
            return Ok(Some(()));
        }
        Some(output) => output,
    };
    let output = output.join(path.file_stem().unwrap()).with_extension("svg");
    match svg::save(&output, &document) {
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

fn subprocess(
    path: &Path,
    characters: &[char],
    document_size: f32,
) -> Result<Option<element::Group>> {
    let mut group = element::Group::new();
    let File { mut fonts } = File::open(path)?;
    let metrics = fonts[0].metrics()?;
    let glyph_size = metrics.ascender - metrics.descender;
    let columns = (characters.len() as f32).sqrt().ceil() as usize;
    let offset = document_size / columns as f32;
    let scale = document_size / columns as f32 / glyph_size;
    for (index, character) in characters.iter().enumerate() {
        let glyph = match fonts[0].draw(*character)? {
            Some(glyph) => glyph,
            _ => return Ok(None),
        };
        let x = index % columns;
        let y = index / columns;
        let transform = format!(
            "translate({}, {}) scale({}) translate({}, {}) scale(1, -1)",
            x as f32 * offset,
            y as f32 * offset,
            scale,
            (glyph_size - glyph.advance_width) / 2.0,
            metrics.ascender,
        );
        let mut glyph = founder::drawing::draw(&glyph);
        glyph.assign("transform", transform);
        group.append(glyph);
    }
    Ok(Some(group))
}
