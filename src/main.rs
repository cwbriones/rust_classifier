#![allow(unstable)]

use std::io::{BufferedReader, File};
use std::ascii::AsciiExt;

use classifier::NaiveBayesClassifier;

mod classifier;
mod counter;

fn main() {
    let data_path = Path::new("./data");
    let labels_path = data_path.join("labels.txt");

    let mut file = File::open(&labels_path).unwrap();
    let mut buf_reader = BufferedReader::new(file);

    let mut classifier = NaiveBayesClassifier::new("spam".to_string(), "not_spam".to_string());
    let mut labeled_features = Vec::new();

    loop {
        if let Ok(line) = buf_reader.read_line() {
            let split_line: Vec<&str> = line.as_slice().split(' ').collect();
            match split_line.as_slice() {
                [label, filename] => {
                    let full_path = data_path.join_many(&["emails", filename.trim()]);
                    let features = extract_features(&full_path);
                    labeled_features.push((label.to_string(), features))
                },
                _ => panic!("This is an error")
            }
        } else {
            break
        }
    }

    println!("Read {} training examples.", labeled_features.len());
    classifier.train(labeled_features);
    println!("Training completed.");
}

// Functions to take raw text and extract normalized features
// Reads tokens from the email file
fn extract_features(path: &Path) -> Vec<String> {
    let mut file = File::open(path).unwrap();

    let data = match file.read_to_string() {
        Ok(s) => s,
        Err(_) => "".to_string()
    };

    // Tokenize
    data.as_slice()
        .to_ascii_lowercase()
        .trim()
        .split_str(" ")
        .map(|s| s.to_string()).collect()
}
