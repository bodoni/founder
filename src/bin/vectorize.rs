mod support;

extern crate arguments;
extern crate colored;
extern crate folder;
extern crate founder;

use std::io::Result;
use std::path::{Path, PathBuf};

use colored::Colorize;
use svg::node::element;
use svg::Document;

fn main() {
    let arguments = arguments::parse(std::env::args()).unwrap();
    let path: PathBuf = arguments
        .get::<String>("path")
        .unwrap_or_else(|| ".".to_string())
        .into();
    let characters = arguments
        .get::<String>("characters")
        .unwrap_or_else(|| "BESbswy".to_string());
    let excludes = arguments.get_all::<String>("exclude").unwrap_or_default();
    let excludes = excludes.iter().map(String::as_str).collect::<Vec<_>>();
    support::summarize(
        &folder::scan(
            &path,
            |path| support::filter(path, &[".otf", ".ttf"], &excludes),
            process,
            characters,
            arguments.get::<usize>("workers"),
        )
        .collect::<Vec<_>>(),
    );
}

fn process(path: &Path, characters: String) -> Result<Option<()>> {
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
                let character = format!("{}-{:#x}", character, character as usize);
                let path = path.parent().unwrap().join(path.file_stem().unwrap());
                std::fs::create_dir_all(&path)?;
                let path = path.join(character).with_extension("svg");
                let mut file = File::create(path)?;
                write!(file, "{document}")?;
                option = Some(());
            }
            eprintln!("{} {path:?}", "[success]".green());
            Ok(option)
        }
        Err(error) => {
            eprintln!("{} {path:?} ({error:?})", "[failure]".red());
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

    const REFERENCES: &[char; 2] = &['X', '0'];
    let File { mut fonts } = File::open(path)?;
    let metrics = fonts[0].metrics()?;
    let mut reference = None;
    for character in REFERENCES.iter() {
        reference = fonts[0].glyph(*character)?;
        if reference.is_some() {
            break;
        }
    }
    let mut results = vec![];
    for character in characters.chars() {
        let (reference, glyph) = match (reference.as_ref(), fonts[0].glyph(character)?) {
            (Some(reference), Some(glyph)) => (reference, glyph),
            _ => {
                results.push((character, None));
                continue;
            }
        };
        let (x, y, scale) = founder::drawing::transform(
            &glyph,
            &metrics,
            reference,
            document_size - 2.0 * margin_size,
        );
        let transform = format!(
            "translate({margin_size} {margin_size}) scale({scale}) translate({x} {y}) scale(1 -1)",
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
