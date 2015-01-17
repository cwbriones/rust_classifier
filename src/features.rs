use std::io::{BufferedReader, File};
use std::ascii::AsciiExt;
use std::rand::{thread_rng, Rng};

type Example = Vec<String>;
type LabeledExample = (bool, Example);

pub fn load_all_examples(data_dir: &str) -> Vec<LabeledExample> {
    let data_path = Path::new(data_dir);
    let labels_path = data_path.join("labels.txt");

    let mut file = File::open(&labels_path).unwrap();
    let mut buf_reader = BufferedReader::new(file);
    let mut labeled_features = Vec::new();

    loop {
        if let Ok(line) = buf_reader.read_line() {
            let split_line: Vec<&str> = line.as_slice().split(' ').collect();
            match split_line.as_slice() {
                [label, filename] => {
                    let full_path = data_path.join_many(&["emails", filename.trim()]);
                    let features = extract_features(&full_path);

                    let mark = match label {
                        "spam" => true,
                        "not_spam" => false,
                        _ => panic!("Found unexpected label {}", label)
                    };
                    labeled_features.push((mark, features))
                },
                _ => panic!("This is an error")
            }
        } else {
            let mut rng = thread_rng();
            rng.shuffle(labeled_features.as_mut_slice());

            return labeled_features;
        }
    }
}

// Functions to take raw text and extract normalized features
// Reads tokens from the email file
fn extract_features(path: &Path) -> Example {
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
