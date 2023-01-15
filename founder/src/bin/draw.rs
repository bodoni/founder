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
            eprintln!("Error: --path should be given.");
            return;
        }
    };
    let characters = match arguments.get::<String>("characters") {
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

fn process(path: &Path, (characters, output): (String, Option<PathBuf>)) -> Result<Option<()>> {
    use std::fs::File;
    use std::io::Write;

    const DOCUMENT_SIZE: f32 = 512.0;
    const MARGIN_SIZE: f32 = 8.0;
    match subprocess(path, &characters, DOCUMENT_SIZE, MARGIN_SIZE) {
        Ok(results) => {
            let mut option = None;
            for (character, document) in results
                .into_iter()
                .filter(|(_, option)| option.is_some())
                .map(|(character, option)| (character, option.unwrap()))
            {
                match output {
                    Some(ref output) => {
                        let character = format!("{}-{:#x}", character, character as usize);
                        let output = output.join(path.file_stem().unwrap());
                        std::fs::create_dir_all(&output)?;
                        let output = output.join(character).with_extension("svg");
                        let mut file = File::create(output)?;
                        write!(file, "{}", document)?;
                    }
                    _ => eprintln!("{}", document),
                }
                option = Some(());
            }
            eprintln!("[success] {:?}", path);
            Ok(option)
        }
        Err(error) => {
            eprintln!("[failure] {:?}", path);
            Err(error)
        }
    }
}

fn subprocess(
    path: &Path,
    characters: &str,
    document_size: f32,
    margin_size: f32,
) -> Result<Vec<(char, Option<element::SVG>)>> {
    use font::File;

    let File { mut fonts } = File::open(path)?;
    let mut results = vec![];
    for character in characters.chars() {
        let glyph = match fonts[0].draw(character)? {
            Some(glyph) => glyph,
            _ => {
                results.push((character, None));
                continue;
            }
        };
        let (left, bottom, right, top) = glyph.bounding_box;
        let glyph_size = (right - left).max(top - bottom);
        let scale = (document_size - 2.0 * margin_size) / glyph_size;
        let transform = format!(
            "scale({}) translate({}, {}) scale(1, -1)",
            scale,
            -left + (glyph_size - (right - left)) / 2.0 + margin_size,
            top + (glyph_size - (top - bottom)) / 2.0 + margin_size,
        );
        let glyph = founder::drawing::draw(&glyph).set("transform", transform);
        let style = element::Style::new("path { fill: black; fill-rule: nonzero; }");
        let document = Document::new()
            .set("width", document_size)
            .set("height", document_size)
            .add(style)
            .add(glyph);
        results.push((character, Some(document)));
    }
    Ok(results)
}
