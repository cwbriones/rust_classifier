#![allow(unstable)]

use classifier::NaiveBayesClassifier;

mod classifier;
mod counter;
mod features;

fn main() {
    let mut classifier = NaiveBayesClassifier::new();

    let labeled_features = features::load_all_examples("./data");
    println!("Read {} training examples.", labeled_features.len());

    classifier.train(labeled_features);
    println!("Training completed.");
}
