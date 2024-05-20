mod support;

use std::io::Result;
use std::path::{Path, PathBuf};

use colored::Colorize;
use font::opentype::truetype::Tag;

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
    use std::io::Write;

    match subprocess(path) {
        Ok(result) => {
            {
                let path = path
                    .parent()
                    .unwrap()
                    .join(path.file_stem().unwrap())
                    .with_extension("txt");
                let mut file = std::fs::File::create(path)?;
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
    use std::fmt::Write;

    let font::File { mut fonts } = font::File::open(path)?;
    let mut string = String::new();
    for (feature, value) in fonts[0].features()? {
        let feature = Tag::from(feature);
        let feature = feature.as_str().unwrap_or("<none>");
        for (script, value) in value.scripts {
            let script = Tag::from(script);
            let script = script.as_str().unwrap_or("<none>");
            for language in value {
                let language = language.map(Tag::from);
                let language = language.as_ref().and_then(Tag::as_str).unwrap_or("<none>");
                writeln!(string, "{feature: <10} {script: <10} {language}").unwrap();
            }
        }
    }
    Ok(string)
}
