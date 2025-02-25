use std::{path::Path, process};
use toj::compute_model;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    leaf_file_path: String,

    #[arg(short, long, default_value_t = false, help = "See details")]
    verbose: bool,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Traverse to root of file system"
    )]
    skip_empty: bool,
}

fn main() {
    let args = Args::parse();
    let leaf_file_path = Path::new(&args.leaf_file_path);

    if !leaf_file_path.exists() || !leaf_file_path.is_file() {
        eprintln!("Error, invalid file: {}", leaf_file_path.to_str().unwrap());
        process::exit(1);
    }

    let merged_model = compute_model(leaf_file_path, args.verbose, args.skip_empty);

    println!("{}", serde_json::to_string_pretty(&merged_model).unwrap());
}
