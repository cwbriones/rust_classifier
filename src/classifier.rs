use std::num::Float;
use std::collections::HashMap;

struct Counter {
    inner: HashMap<String, u32>
}

impl Counter {
    fn new() -> Counter {
        Counter { inner: HashMap::new() }
    }

    fn get(&self, label: &str) -> u32 {
        if self.inner.contains_key(label) {
            *self.inner.get(label).unwrap()
        } else {
            0
        }
    }

    fn increment(&mut self, label: &str) -> u32 {
        if self.inner.contains_key(label) {
            let current = self.inner.get_mut(label).unwrap();
            *current += 1;
            *current
        } else {
            self.inner.insert(label.to_string(), 1);
            1
        }
    }
}

pub struct NaiveBayesClassifier {
    label_feature_lookup: HashMap<String, Counter>,
    label_total_feature_counts: Counter,
    label_total_document_counts: Counter,
    label1: String,
    label2: String
}

impl NaiveBayesClassifier {
    pub fn new(label1: String, label2: String) -> NaiveBayesClassifier {
        let mut label_feature_lookup = HashMap::new();

        label_feature_lookup.insert(label1.clone(), Counter::new());
        label_feature_lookup.insert(label2.clone(), Counter::new());

        NaiveBayesClassifier {
            label_feature_lookup: label_feature_lookup,
            label_total_feature_counts: Counter::new(),
            label_total_document_counts: Counter::new(),
            label1: label1,
            label2: label2
        }
    }

    pub fn train(&mut self, labeled_features: Vec<(String, Vec<String>)>) {
        for &(ref label, ref feature_vec) in labeled_features.iter() {
            self.label_total_document_counts.increment(label.as_slice());

            for feature in feature_vec.iter() {
                let mut feature_count = self.label_feature_lookup.get_mut(label).unwrap();

                feature_count.increment(feature.as_slice());
                self.label_total_feature_counts.increment(feature.as_slice());
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
            (self.label_total_document_counts.get(label1) as f64 + 1.0) /
            (self.label_total_document_counts.get(label2) as f64 + 1.0));
        prior + total_weight > 0.0
    }

    fn p_feature_given_label(&self, feature: &str, label: &str) -> f64 {
        let num = self.label_feature_lookup.get(label).unwrap().get(feature) + 1;
        let den = self.label_total_feature_counts.get(label) + 1;

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

