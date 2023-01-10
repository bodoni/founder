extern crate arguments;
extern crate font;
extern crate founder;
extern crate svg;

use std::io::Result;
use std::path::{Path, PathBuf};

use svg::node::element;
use svg::Document;

fn main() {
    let arguments = arguments::parse(std::env::args()).unwrap();
    let path: PathBuf = match arguments.get::<String>("path") {
        Some(path) => path.into(),
        _ => {
            println!("Error: --path should be given.");
            return;
        }
    };
    let character = match arguments.get::<String>("character") {
        Some(character) => character.chars().next().unwrap(),
        _ => {
            println!("Error: --character should be given.");
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
        (character, output),
        arguments.get::<usize>("workers").unwrap_or(1),
        &arguments.get_all::<String>("ignore").unwrap_or(vec![]),
    );
}

fn process(
    path: PathBuf,
    (character, output): (char, Option<PathBuf>),
) -> (PathBuf, Result<Option<()>>) {
    use std::fs::File;
    use std::io::Write;

    let result = match subprocess(&path, character) {
        Ok(Some(result)) => match output {
            Some(output) => {
                let output = output.join(path.file_stem().unwrap()).with_extension("svg");
                let mut file = File::create(output).unwrap();
                write!(file, "{}", result).unwrap();
                Ok(Some(()))
            }
            _ => {
                println!("{}", result);
                Ok(Some(()))
            }
        },
        Ok(None) => Ok(None),
        Err(error) => Err(error),
    };
    (path, result)
}

fn subprocess(path: &Path, character: char) -> Result<Option<element::SVG>> {
    use font::File;

    let File { mut fonts } = File::open(path)?;
    let metrics = fonts[0].metrics()?;
    let glyph = match fonts[0].draw(character)? {
        Some(glyph) => glyph,
        _ => return Ok(None),
    };
    let (width, height) = (
        glyph.width() + 2.0 * glyph.side_bearings.0,
        metrics.ascender - metrics.descender,
    );
    let transform = format!("translate(0, {}) scale(1, -1)", metrics.ascender);
    let glyph = founder::drawing::draw(&glyph).set("transform", transform);
    let style = element::Style::new("path { fill: black; fill-rule: nonzero; }");
    Ok(Some(
        Document::new()
            .set("width", width)
            .set("height", height)
            .add(style)
            .add(glyph),
    ))
}
