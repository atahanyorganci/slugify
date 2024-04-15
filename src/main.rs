use std::path::{Component, Path, MAIN_SEPARATOR};

use clap::Parser;
use slug;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    input: String,
    #[clap(short, long)]
    delim: Option<char>,
    #[clap(short, long)]
    path: bool,
}

fn slugify(input: &str, delim: Option<char>) -> String {
    let result = slug::slugify(input);
    match delim {
        Some(delim) if delim != '-' => result.replace('-', &delim.to_string()),
        _ => result,
    }
}
fn slugify_path(path: &Path, delim: Option<char>) {
    let parts = path.components().collect::<Vec<_>>();
    let count = parts.len();
    for i in 0..count - 1 {
        match parts[i] {
            Component::Prefix(_) => unreachable!(),
            Component::RootDir => print!("{}", MAIN_SEPARATOR),
            Component::CurDir => print!(".{}", MAIN_SEPARATOR),
            Component::ParentDir => print!("..{}", MAIN_SEPARATOR),
            Component::Normal(part) => {
                let part = part.to_str().unwrap();
                let slug = slugify(part, delim);
                print!("{slug}{MAIN_SEPARATOR}");
            }
        }
    }
    match parts[count - 1] {
        Component::Prefix(_) => unreachable!(),
        Component::RootDir => print!("{}", MAIN_SEPARATOR),
        Component::CurDir => print!("."),
        Component::ParentDir => print!(".."),
        Component::Normal(part) => {
            let part = part.to_str().unwrap();
            if let Some(ext) = Path::new(part).extension().map(|ext| ext.to_str().unwrap()) {
                let slug = slugify(part.strip_suffix(ext).unwrap(), delim);
                print!("{slug}.{ext}");
            } else {
                let slug = slugify(part, delim);
                print!("{}", slug);
            }
        }
    }
    println!();
}

fn main() {
    let Cli { input, delim, path } = Cli::parse();
    if input == "-" {
        todo!("Read from stdin")
    } else {
        if path {
            let path = Path::new(&input);
            slugify_path(path, delim)
        } else {
            println!("{}", slugify(&input, delim));
        }
    }
}
