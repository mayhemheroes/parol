use honggfuzz::fuzz;
use json_parser::{json_parser::parse, json_grammar::JsonGrammar};

fn main() {
    let mut json_grammar = JsonGrammar::new();
    loop {
        fuzz!(|data: &[u8]| {
            let source: &str = std::str::from_utf8(data).unwrap();
            let file_name = "fuzz.json".to_string();
            let _ = parse(source, &file_name, &mut json_grammar);
        });
    }
}