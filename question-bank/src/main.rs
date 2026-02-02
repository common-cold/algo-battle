use std::{env, fs};

use crate::question_generator::QuestionGenerator;

mod question_generator;

fn main() {
    let path = env::args()
        .nth(1)
        .expect("Usage: cargo run questions/<folder-name>");

    let question_name = path.rsplit("/").next().unwrap().to_string();
    println!("QUESTION NAME: {}",question_name);


    let structure_path = format!("{}/Structure.md", path);
    let cpp_path_full = format!("{}/full/{}_full.cpp", path, question_name);
    let js_path_full = format!("{}/full/{}_full.js", path, question_name);
    let rust_path_full = format!("{}/full/{}_full.rs", path, question_name);

    let cpp_path = format!("{}/partial/{}.cpp", path, question_name);
    let js_path = format!("{}/partial/{}.js", path, question_name);
    let rust_path = format!("{}/partial/{}.rs", path, question_name);

    let contents = fs::read_to_string(structure_path).expect("File not found");

    let generator = QuestionGenerator::new(contents).unwrap();

    
    generator.generate_cpp_code_partial(cpp_path).unwrap();
    generator.generate_cpp_code_full(cpp_path_full).unwrap();

    generator.generate_js_code_partial(js_path).unwrap();
    generator.generate_js_code_full(js_path_full).unwrap();

    generator.generate_rust_code_partial(rust_path).unwrap();
    generator.generate_rust_code_full(rust_path_full).unwrap();
}