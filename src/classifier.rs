use std::num::Float;
use std::collections::HashMap;

use counter::Counter;

pub struct NaiveBayesClassifier {
    positive_label: Label,
    negative_label: Label,
    total_feature_count: Counter,
}

#[derive(Clone)]
struct Label {
    feature_count: u64,
    document_count: u64,
    feature_lookup: Counter,
}

impl NaiveBayesClassifier {
    pub fn new() -> NaiveBayesClassifier {
        let positive = Label {
            feature_count: 0,
            document_count: 0,
            feature_lookup: Counter::new(),
        };
        let negative = positive.clone();

        NaiveBayesClassifier {
            positive_label: positive,
            negative_label: negative,
            total_feature_count: Counter::new(),
        }
    }

    pub fn train(&mut self, labeled_features: &[(bool, Vec<String>)]) {
        for &(label, ref feature_vec) in labeled_features.iter() {
            if label {
                self.positive_label.document_count += 1;
            } else {
                self.negative_label.document_count += 1;
            }

            for feature in feature_vec.iter() {
                let feature_count = match label {
                    true => &mut self.positive_label.feature_lookup,
                    false => &mut self.negative_label.feature_lookup
                };

                feature_count.increment(feature.as_slice());
                self.total_feature_count.increment(feature.as_slice());
            }
        }
    }

    pub fn classify(&self, feature_vec: &[String]) -> bool {
        let mut total_weight = 0.0;

        for feature in feature_vec.iter() {
            let p_pos = self.p_feature_given_label(feature.as_slice(), true) as f64;
            let p_neg = self.p_feature_given_label(feature.as_slice(), false) as f64;

            total_weight += Float::log2(p_pos/p_neg);
        }

        let prior = Float::log2(
            (self.positive_label.document_count as f64 + 1.0) /
            (self.negative_label.document_count as f64 + 1.0));
        prior + total_weight > 0.0
    }

    fn p_feature_given_label(&self, feature: &str, label: bool) -> f64 {
        let mut label = match label {
            true  => &self.positive_label,
            false => &self.negative_label
        };
        let num = label.feature_lookup.get(feature) + 1;
        let den = label.feature_count + 1;

        (num as f64) / (den as f64)
    }
}
