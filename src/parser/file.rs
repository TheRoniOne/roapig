use std::fs::read_to_string;

fn parse_file(path: &str) {
    let content = read_to_string(path).unwrap();

    content;
}
