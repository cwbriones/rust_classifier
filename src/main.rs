#![allow(unstable)]
use std::io::{BufferedReader, File};
use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::num::Float;

type Counter<K> = HashMap<K, u32>;

struct NaiveBayesClassifier {
    label_feature_lookup: HashMap<String, Counter<String>>,
    label_total_feature_counts: Counter<String>,
    label_total_document_counts: Counter<String>,
    label1: String,
    label2: String
}

impl NaiveBayesClassifier {
    pub fn new(label1: String, label2: String) -> NaiveBayesClassifier {
        let mut label_total_document_counts = HashMap::new();
        let mut label_feature_lookup = HashMap::new();

        label_feature_lookup.insert(label1.clone(), HashMap::new());
        label_feature_lookup.insert(label2.clone(), HashMap::new());

        label_total_document_counts.insert(label1.clone(), 0);
        label_total_document_counts.insert(label2.clone(), 0);

        NaiveBayesClassifier {
            label_feature_lookup: label_feature_lookup,
            label_total_document_counts: label_total_document_counts,
            label_total_feature_counts: HashMap::new(),
            label1: label1,
            label2: label2
        }
    }

    pub fn train(&mut self, labeled_features: Vec<(String, Vec<String>)>) {
        for &(ref label, ref feature_vec) in labeled_features.iter() {
            *self.label_total_document_counts.get_mut(label).unwrap() += 1;

            for feature in feature_vec.iter() {
                let mut feature_count = self.label_feature_lookup.get_mut(label).unwrap();

                if feature_count.contains_key(feature) {
                    *feature_count.get_mut(feature).unwrap() += 1;
                    *self.label_total_feature_counts.get_mut(feature).unwrap() += 1;
                } else {
                    feature_count.insert(feature.clone(), 1);
                    self.label_total_feature_counts.insert(feature.clone(), 1);
                }
            }
        }
    }

    fn classify(&self, feature_vec: Counter<String>, label1: &str, label2: &str) -> bool {
        let mut total_weight = 0.0;

        for (feature, _) in feature_vec.iter() {
            let p1 = self.p_feature_given_label(feature.as_slice(), label1) as f64;
            let p2 = self.p_feature_given_label(feature.as_slice(), label1) as f64;

            total_weight += Float::log2(p1/p2);
        }

        let prior = Float::log2(
            (*self.label_total_document_counts.get(label1).unwrap() as f64 + 1.0) /
            (*self.label_total_document_counts.get(label2).unwrap() as f64 + 1.0));
        prior + total_weight > 0.0
    }

    fn p_feature_given_label(&self, feature: &str, label: &str) -> f64 {
        let num = *self.label_feature_lookup.get(label).unwrap().get(feature).unwrap() + 1;
        let den = *self.label_total_feature_counts.get(label).unwrap() + 1;

        (num as f64) / (den as f64)
    }
}

/*fn increment(key: &String, counter: &mut Counter<String>) -> u32 {*/
/*    if !counter.contains_key(key) {*/
/*        counter.insert(key, 1);*/
/*        1*/
/*    } else {*/
/*        let current = counter.get_mut(key).unwrap();*/
/*        *current += 1;*/
/*        current*/
/*    }*/
/*}*/

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

//Functions to take raw text and extract normalized features
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
