mod support;

extern crate arguments;
extern crate colored;
extern crate folder;
extern crate founder;

use std::io::Result;
use std::path::{Path, PathBuf};

use colored::Colorize;

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
            |path| support::filter(path, &[".otf", ".ttf"], &excludes),
            process,
            (),
            arguments.get::<usize>("workers"),
        )
        .collect::<Vec<_>>(),
    );
}

fn process(path: &Path, _: ()) -> Result<Option<()>> {
    use std::fs::File;
    use std::io::Write;

    match subprocess(path) {
        Ok(result) => {
            {
                let path = path
                    .parent()
                    .unwrap()
                    .join(path.file_stem().unwrap())
                    .with_extension("txt");
                let mut file = File::create(path)?;
                write!(file, "{result}")?;
            }
            eprintln!("{} {path:?}", "[success]".green());
            Ok(Some(()))
        }
        Err(error) => {
            eprintln!("{} {path:?} ({error:?})", "[failure]".red());
            Err(error)
        }
    }
}

fn subprocess(path: &Path) -> Result<String> {
    use font::File;
    use std::fmt::Write;

    let File { mut fonts } = File::open(path)?;
    let mut string = String::new();
    let table = fonts[0].names()?;
    let table = table.borrow();
    let language_tags = table.language_tags().collect::<Vec<_>>();
    for ((_, _, language_id, name_id), value) in table.iter() {
        let name_id = format!("{name_id:?}");
        let language_tag = language_id.tag(&language_tags).unwrap_or("--");
        let value = truncate(value.as_deref().unwrap_or("--"));
        writeln!(string, "{name_id: <25} {language_tag: <5} {value}").unwrap();
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
        string.push('…');
    }
    string
}
