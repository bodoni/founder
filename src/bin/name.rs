extern crate arguments;
extern crate font;
extern crate founder;

use std::io::Result;
use std::path::{Path, PathBuf};

fn main() {
    let arguments = arguments::parse(std::env::args()).unwrap();
    let path: PathBuf = match arguments.get::<String>("path") {
        Some(path) => path.into(),
        _ => {
            println!("Error: --path should be given.");
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
        output,
        arguments.get::<usize>("workers").unwrap_or(1),
        &arguments.get_all::<String>("ignore").unwrap_or(vec![]),
    );
}

fn process(path: &Path, output: Option<PathBuf>) -> Result<Option<()>> {
    use std::fs::File;
    use std::io::Write;

    match subprocess(path) {
        Ok(result) => match output {
            Some(output) => {
                let output = output.join(path.file_stem().unwrap()).with_extension("txt");
                let mut file = File::create(output).unwrap();
                write!(file, "{}", result).unwrap();
                Ok(Some(()))
            }
            _ => {
                println!("{}", result);
                Ok(Some(()))
            }
        },
        Err(error) => Err(error),
    }
}

fn subprocess(path: &Path) -> Result<String> {
    use font::File;
    use std::fmt::Write;

    let File { mut fonts } = File::open(path)?;
    let mut string = String::new();
    for ((name_id, language_tag), value) in fonts[0].names()?.iter() {
        let name_id = format!("{:?}", name_id);
        let language_tag = language_tag.as_deref().unwrap_or("--");
        let value = truncate(value.as_deref().unwrap_or("--"));
        writeln!(string, "{: <25} {: <5} {}", name_id, language_tag, value).unwrap();
    }
    Ok(string)
}

fn truncate(string: &str) -> String {
    const MAX: usize = 50;
    let count = string.chars().count();
    let mut string = match string.char_indices().nth(MAX) {
        None => string.to_owned(),
        Some((index, _)) => string[..index].to_owned(),
    };
    if count > MAX {
        string.push_str("…");
    }
    string
}
