use std::path::PathBuf;
use std::process::Command;

macro_rules! exit(
    ($code:expr) => (std::process::exit($code));
);

macro_rules! some(
    ($option:expr) => (match $option {
        Some(value) => value,
        None => {
            eprintln!(concat!("Error: failed to unwrap (", stringify!($option), ")."));
            exit!(1);
        }
    });
);

fn main() {
    let arguments: Vec<_> = std::env::args().collect();
    let prefix = PathBuf::from(some!(arguments.get(0)));
    let prefix = some!(some!(prefix.file_name()).to_str());
    if arguments.len() > 1 {
        let program = format!("{prefix}-{}", arguments[1]);
        let arguments = arguments.iter().skip(2).collect::<Vec<_>>();
        match Command::new(program).args(arguments).status() {
            Ok(value) => {
                exit!(value.code().unwrap_or(0));
            }
            Err(error) => {
                eprintln!("Error: {error}.");
                exit!(1);
            }
        }
    } else {
        eprintln!("Usage: {prefix} [command] [arguments]");
        exit!(1);
    }
}
