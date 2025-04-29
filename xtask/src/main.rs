#![warn(rust_2018_idioms)]

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
enum Args {
    Coverage {
        #[arg(short = 'I', long)]
        api_dir: PathBuf,
        #[arg(short, long)]
        print: bool,
        #[arg(short, long)]
        verbose: bool,
    },
}

mod coverage;

fn main() {
    color_eyre::install().unwrap();

    let args = Args::parse();
    match args {
        Args::Coverage {
            api_dir,
            print,
            verbose,
        } => {
            let core_include_dir = api_dir.join("core").join("inc");
            let studio_include_dir = api_dir.join("studio").join("inc");

            if let Err(e) = coverage::coverage(core_include_dir, studio_include_dir, print, verbose)
            {
                eprintln!("Error: {e:?}");
            }
        }
    }
}
