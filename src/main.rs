use clap::Parser;
use slug;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    input: String,
    #[clap(short, long)]
    delim: Option<char>,
}

fn slugify(input: &str, delim: Option<char>) -> String {
    let result = slug::slugify(input);
    match delim {
        Some(delim) if delim != '-' => result.replace('-', &delim.to_string()),
        _ => result,
    }
}

fn main() {
    let Cli { input, delim } = Cli::parse();
    if input == "-" {
        todo!("Read from stdin")
    } else {
        println!("{}", slugify(&input, delim));
    }
}
