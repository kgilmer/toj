use serde_json::Value;
use std::error::Error;
use std::{fs::File, io::BufReader, path::Path, process};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    leaf_file_path: String,

    #[arg(short, long, default_value_t = false, help = "See details")]
    verbose: bool,

    #[arg(short, long, default_value_t = false, help = "Traverse to root of file system")]
    skip_empty: bool,
}

fn main() {
    let args = Args::parse();
    let leaf_file_path = Path::new(&args.leaf_file_path);

    if !leaf_file_path.exists() || !leaf_file_path.is_file() {
        eprintln!(
            "Error, invalid file: {}",
            leaf_file_path.to_str().unwrap()
        );
        process::exit(1);
    }

    let filename = leaf_file_path.file_name().expect("Validated file");

    let leaf_path = leaf_file_path
        .parent()
        .expect("Validated file path")
        .parent()
        .expect("Validated file path");

    let mut model_dirs = vec![leaf_file_path.to_path_buf()];
    if args.verbose {
        println!("Found model: {:?}", &leaf_file_path);
    }

    for segment in leaf_path.ancestors() {
        let mut pb = segment.to_path_buf();
        pb.push(filename);

        if pb.exists() {
            if args.verbose {
                println!("Found model: {:?}", &pb);
            }
            model_dirs.push(pb);
        } else if !args.skip_empty {
            break;
        }
    }

    model_dirs.reverse(); // Merge from parent to child, child overriding parent

    let mut model_iter = model_dirs.iter();
    let merged_model = model_iter.next().expect("Known to have at least one model");
    let mut merged_model = read_json_file(merged_model).expect("Can read and parse json file");

    for child_model_path in model_iter {
        let child_model = read_json_file(child_model_path).expect("Can read and parse json file");
        merge(&mut merged_model, child_model);
    }

    println!("{}", serde_json::to_string_pretty(&merged_model).unwrap());
}

fn read_json_file<P: AsRef<Path>>(path: P) -> Result<Value, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let u = serde_json::from_reader(reader)?;

    Ok(u)
}

fn merge(a: &mut Value, b: Value) {
    if let Value::Object(a) = a {
        if let Value::Object(b) = b {
            for (k, v) in b {
                if v.is_null() {
                    a.remove(&k);
                } else {
                    merge(a.entry(k).or_insert(Value::Null), v);
                }
            }

            return;
        }
    }

    *a = b;
}
