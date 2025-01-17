#![allow(dead_code)]
use jsonschema::Draft;
use serde_json::Value;
use std::fs;
use std::path::Path;
mod models;

/// Validate AsyncAPI Spec file
///
/// # Arguments
///
/// * `filepath` - path of the AsycnAPI specification file.
///
/// # Returns
///
/// A new `Result` instance.
pub fn validate(filepath: &str) -> Result<(), Vec<String>> {
    let spec_json = load_spec(filepath);
    let scheme = load_scheme();
    let compiled_scheme = jsonschema::options()
        .with_draft(Draft::Draft7)
        .build(&scheme)
        .expect("error building scheme");
    let is_valid = compiled_scheme.is_valid(&spec_json);
    if !is_valid {
        let errors: Vec<String> = compiled_scheme
            .iter_errors(&spec_json)
            .map(|s| s.to_string())
            .collect();
        return Err(errors);
    }
    Ok(())
}

fn load_spec(filepath: &str) -> Value {
    let file_path = Path::new(filepath);
    let file_conent = read_file(filepath);
    let extension = get_extension(file_path);
    let spec_json: Value = match extension {
        Some(ext) => {
            let json_data: Value = if ext == "json" {
                serde_json::from_str(&file_conent).expect("error parsing spec file")
            } else if ext == "yaml" || ext == "yml" {
                serde_yaml::from_str(&file_conent).expect("error parsing spec file")
            } else {
                panic!("Invalid file format")
            };
            json_data
        }
        None => panic!(),
    };
    spec_json
}

fn load_scheme() -> Value {
    let filepath = "./3.0.0-scheme.json".to_string();
    let path: &Path = Path::new(&filepath);
    let scheme_content = fs::read_to_string(path).expect("error reading scheme file");
    let scheme_json: Value =
        serde_json::from_str(&scheme_content).expect("error parsing scheme file");
    scheme_json
}

fn read_file(filepath: &str) -> String {
    let path: &Path = Path::new(filepath);
    fs::read_to_string(path).expect("error loading file from the given path")
}

fn get_extension(path: &Path) -> Option<&str> {
    let extension = path.extension().and_then(|ext| ext.to_str());
    extension
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_spec_file() {
        let result = load_spec("./asyncapi.yml");
        if let Some(asyncapi) = result.get("asyncapi").and_then(|v| v.as_str()) {
            assert_eq!(asyncapi, "3.0.0");
        }
    }

    #[test]
    fn load_scheme_file() {
        let result = load_scheme();
        if let Some(scheme) = result.get("$schema").and_then(|v| v.as_str()) {
            assert_eq!(scheme, "http://json-schema.org/draft-07/schema");
        }
    }

    #[test]
    fn validate_spec() {
        let result = validate("./asyncapi.yml");
        match result {
            Ok(()) => assert!(true),
            Err(e) => assert!(e.is_empty()),
        }
    }
}
