use std::path::Path;

mod dictionary;
mod rule;

use unidecode::unidecode;

use clap::{Arg, Command};

fn should_generate_dictionary(force_generate_dictionary: bool, output_file_exists: bool) -> bool{
    force_generate_dictionary || !output_file_exists
}

fn output_file_exists(output_file_name: &str) -> bool{
    Path::new(output_file_name).exists()
}

fn init_dictionary(force_generate_dictionary: bool) -> Vec<String> {
    let output_file_name = "words-5-letters.txt";

    if should_generate_dictionary(force_generate_dictionary, output_file_exists(output_file_name)) {
        println!("{}", "Dictionary generated");
        dictionary::generate("words.txt", output_file_name);
        return dictionary::fetch_words_with_5_letters("words.txt");
    }else{
        return dictionary::generated_dictionary_of_words_with_5_letters(output_file_name);
    }

}

fn exclude_words_with(letter: char, possible_words: Vec<String>) -> Vec<String>{
    return possible_words.iter().filter(|&word| !unidecode(word).contains(letter)).cloned().collect();

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

        /*
    let json_to_parse = Arg::new("json")
        .takes_value(true)
        .help("JSon with the ruleset to be parsed")
        .required(true);*/

    let app = app.args([force_generate_dictionary]);

    let matches = app.get_matches();

    let words_with_5_letters : Vec<String> = init_dictionary(matches.is_present("generate-dictionary"));
/*
    let json_to_parse_value = matches.value_of("json")
            .expect("Json with the ruleset is required");

    println!("JSON to parse {}", json_to_parse_value);*/

    
    let json_with_rules = r#"
    {
        "rules": 
        [
            {"letter": "a", "rule_type": "missing"}
        ]
    }"#;

    let ruleset: rule::Ruleset = match rule::parse(json_with_rules) {
        Ok(result) => result,
        Err(error) => panic!("Error: {:?}", error),
    };

    
    if ruleset.rules[0].rule_type == "missing"{
        let x = exclude_words_with(ruleset.rules[0].letter, words_with_5_letters);
        println!("Vector: {:?}", x);
    }

}
