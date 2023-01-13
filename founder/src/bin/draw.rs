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
    let characters = match arguments.get::<String>("characters") {
        Some(characters) => characters.chars().collect(),
        _ => {
            println!("Error: --characters should be given.");
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

    match subprocess(path, &characters) {
        Ok(results) => {
            let mut option = None;
            for (character, document) in results
                .into_iter()
                .filter(|(_, option)| option.is_some())
                .map(|(character, option)| (character, option.unwrap()))
            {
                match output {
                    Some(ref output) => {
                        let output = output.join(path.file_stem().unwrap());
                        std::fs::create_dir_all(&output)?;
                        let output = output.join(character.to_string()).with_extension("svg");
                        let mut file = File::create(output)?;
                        write!(file, "{}", document)?;
                    }
                    _ => println!("{}", document),
                }
                option = Some(());
            }
            Ok(option)
        }
        Err(error) => Err(error),
    }
}

fn subprocess(path: &Path, characters: &str) -> Result<Vec<(char, Option<element::SVG>)>> {
    use font::File;

    let File { mut fonts } = File::open(path)?;
    let metrics = fonts[0].metrics()?;
    let mut results = vec![];
    for character in characters.chars() {
        let glyph = match fonts[0].draw(character)? {
            Some(glyph) => glyph,
            _ => {
                results.push((character, None));
                continue;
            }
        };
        let (width, height) = (
            glyph.width() + 2.0 * glyph.side_bearings.0,
            metrics.ascender - metrics.descender,
        );
        let transform = format!("translate(0, {}) scale(1, -1)", metrics.ascender);
        let glyph = founder::drawing::draw(&glyph).set("transform", transform);
        let style = element::Style::new("path { fill: black; fill-rule: nonzero; }");
        let document = Document::new()
            .set("width", width)
            .set("height", height)
            .add(style)
            .add(glyph);
        results.push((character, Some(document)));
    }
    Ok(results)
}
