use std::{fs};
use clap::{Parser, Subcommand};

use crate::question_generator::QuestionGenerator;

mod question_generator;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
    path: String
}

#[derive(Subcommand)]
enum Commands {
    CreateQuestion,
    UploadQuestion
}

#[tokio::main]
async fn main() {
    //cargo run questions/<folder-name>

    let cli = Cli::parse();
    let path = cli.path;

    let question_name = path.rsplit("/").next().unwrap().to_string();

    let structure_path: String = format!("{}/Structure.md", path);
    let problem_path: String = format!("{}/Problem.md", path);

    let cpp_path_full: String = format!("{}/full/{}_full.cpp", path, question_name);
    let js_path_full: String = format!("{}/full/{}_full.js", path, question_name);
    let rust_path_full: String = format!("{}/full/{}_full.rs", path, question_name);
   
    let cpp_path: String = format!("{}/partial/{}.cpp", path, question_name);
    let js_path: String = format!("{}/partial/{}.js", path, question_name);
    let rust_path: String = format!("{}/partial/{}.rs", path, question_name);


    match cli.command {
        Commands::CreateQuestion => {
            let contents = fs::read_to_string(&structure_path).expect("Structure File not found");

            let generator = QuestionGenerator::new(contents).unwrap();

            generator.generate_cpp_code_partial(&cpp_path).unwrap();
            generator.generate_cpp_code_full(&cpp_path_full).unwrap();

            generator.generate_js_code_partial(&js_path).unwrap();
            generator.generate_js_code_full(&js_path_full).unwrap();

            generator.generate_rust_code_partial(&rust_path).unwrap();
            generator.generate_rust_code_full(&rust_path_full).unwrap();
        }
        Commands::UploadQuestion => {
            let contents = fs::read_to_string(&problem_path).expect("Problem File not found");
            QuestionGenerator::save_to_db(contents, path, question_name).await.unwrap();
        }
    }    
    
}