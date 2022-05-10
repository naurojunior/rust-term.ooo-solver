use std::env;
use std::path::Path;
use clap::Parser;

mod dictionary;
mod rule;

use clap::{Arg, Command};

fn should_generate_dictionary(force_generate_dictionary: bool, output_file_exists: bool) -> bool{
    force_generate_dictionary || !output_file_exists
}

fn output_file_exists(output_file_name: &str) -> bool{
    Path::new(output_file_name).exists()
}

fn init_dictionary(force_generate_dictionary: bool) {
    let output_file_name = "words-5-letters.txt";

    if should_generate_dictionary(force_generate_dictionary, output_file_exists(output_file_name)) {
        println!("{}", "Dictionary generated");
        dictionary::generate("words.txt", output_file_name)
    }else{
        println!("{}", "Using already generated dictionary");
    }
}


fn main() {

    let app = Command::new("Term.ooo Solver")
        .version("0.1")
        .about("Gives hints on Term.ooo")
        .author("Nauro Junior");
    
    let force_generate_dictionary = Arg::new("generate-dictionary")
        .short('g')
        .long("generate")
        .takes_value(false)
        .help("Forces the generation of the 5 words dictionary")
        .required(false);

    let json_to_parse = Arg::new("json")
        .takes_value(true)
        .help("JSon with the ruleset to be parsed")
        .required(true);

    let app = app.args([force_generate_dictionary, json_to_parse]);

    let matches = app.get_matches();

    init_dictionary(matches.is_present("generate-dictionary"));


    let json_to_parse_value = matches.value_of("json")
            .expect("Json with the ruleset is required");

    println!("JSON to parse {}", json_to_parse_value);

    /*
    let json_with_rules = r#"
    {
        "ruleset": 
        [
            {"letter": "A", "rule": "missing"},
            {"letter": "U", "rule": "missing"},
            {"letter": "D", "rule": "correct"},
            {"letter": "I", "rule": "missing"},
            {"letter": "O", "rule": "misplace"}
        ]
    }"#;*/

    let _ = match rule::parse(json_to_parse_value) {
        Ok(result) => result,
        Err(error) => panic!("Error: {:?}", error),
    };
}
