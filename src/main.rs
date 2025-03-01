#![warn(missing_docs)]

//! Model projections from hierarchical parents in JSON
//! `toj` is a command-line tool that merges JSON documents in an assumed directory structure, starting from a parent document down to any number of specializations in subdirectories.  
//! Combining directory traversal with JSON merge logic, this simple tool allows for patterns like single-parent object-orientated inheritance, but for data. This may be useful in reducing redundancy in cases where a large number of model instances can be realized from a base general model in addition to one or more successive layers of specialization.
//! The merge strategy is to create the model from the top-most parent and then evaluate children in subdirectory order.  Children may Create (add new key/values), Update (specify new values for existing keys), and Delete (set a key's value to `null`) from parent data.
use std::{path::Path, process};
use toj::compute_model;

use clap::Parser;

/// Program args
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

/// CLI bootstrap
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
