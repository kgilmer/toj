use serde_json::Value;
use std::error::Error;
use std::{fs::File, io::BufReader, path::Path};

/// Compute model by merging from the top-most parent to leaf JSON node
/// # Arguments
///
/// * `leaf_file_path` - Path to most nested JSON document
/// * `verbose` - emit additional infor to `stdout`
/// * `skip_empty` - if `true` continue traversing file path to root of filesystem, otherwise stop in the first directory
pub fn compute_model(leaf_file_path: &Path, verbose: bool, skip_empty: bool) -> Value {
    let filename = leaf_file_path.file_name().expect("Validated file");

    let leaf_path = leaf_file_path
        .parent()
        .expect("Validated file path")
        .parent()
        .expect("Validated file path");

    let mut model_dirs = vec![leaf_file_path.to_path_buf()];
    if verbose {
        println!("Found model: {:?}", &leaf_file_path);
    }

    for segment in leaf_path.ancestors() {
        let mut pb = segment.to_path_buf();
        pb.push(filename);

        if pb.exists() {
            if verbose {
                println!("Found model: {:?}", &pb);
            }
            model_dirs.push(pb);
        } else if !skip_empty {
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

    merged_model
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_model_success() {
        let dir_path = Path::new("examples/animal-kingdom/forest/alpine/animal-model.json");

        // Verify that compute_model returns the expected merged result
        let result = compute_model(dir_path, true, false);

        assert_eq!(result.to_string(), "{\"animals\":{\"deer\":{\"avg-weight-kg\":40,\"diet\":\"herbivore\",\"leg-count\":4}}}")
    }
}
