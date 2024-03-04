
use tree_sitter::Parser;

fn main() {
    let source_code = "fn main() {}\n".repeat(100000).to_string();
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_rust::language())
        .expect("Error loading Rust grammar");
    let tree = parser.parse(source_code, None).unwrap();

    drop(tree);
    drop(parser);
    loop {
        // Sleep every 10 miliseconds to prevent high CPU usage
        let ten_millis = std::time::Duration::from_millis(10);
        std::thread::sleep(ten_millis);
        
    }
}

