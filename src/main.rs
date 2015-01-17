#![allow(unstable)]

use classifier::NaiveBayesClassifier;

mod classifier;
mod counter;
mod features;

static SPLIT: f32 = 0.75;

fn main() {

    let all_examples = features::load_all_examples("./data");

    let split_point = (SPLIT * all_examples.len() as f32) as usize;
    let (training, validation) = all_examples.as_slice().split_at(split_point);

    println!("Read {} training examples.", training.len());
    let mut classifier = NaiveBayesClassifier::new();
    classifier.train(training);
    println!("Training completed.");

    let mut true_positives = 0;
    let mut true_negatives = 0;
    let mut false_positives = 0;
    let mut false_negatives = 0;

    println!("Evaluating with {} validation examples", validation.len());
    for &(expected, ref example) in validation.iter() {
        let classified = classifier.classify(example.as_slice());
        match (expected, classified) {
            (true, true) => true_positives += 1,
            (false, false) => true_negatives += 1,
            (false, true) => false_positives += 1,
            (true, false) => false_negatives += 1,
        }
    }

    let precision = (true_positives as f64)/((true_positives + false_positives) as f64);
    let recall    = (true_positives as f64)/((true_positives + false_negatives) as f64);
    let accuracy  = ((true_positives + true_negatives) as f64)/(validation.len() as f64);
    let specific  = (true_negatives as f64)/((true_negatives + false_positives) as f64);

    println!("");
    println!("Results");
    println!("---------------------------");
    println!("True positive:\t{}", true_positives);
    println!("True negative:\t{}", true_negatives);
    println!("False positive:\t{}", false_positives);
    println!("False negative:\t{}", false_negatives);
    println!("---------------------------");
    println!("Precision:\t{}", precision);
    println!("Recall:\t{}", recall);
    println!("Specificity:\t{}", specific);
    println!("Accuracy:\t{}", accuracy);
}
