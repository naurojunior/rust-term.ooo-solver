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

fn word_with_rules(possible_word : String, rules_missing_letters: &Vec<rule::Rule>) -> bool{
    possible_word.chars()
                 .any(|possible_word_letter| rules_missing_letters.into_iter()
                                                                  .filter(|rule| possible_word_letter == rule.letter)
                                                                  .count() > 0)
}

fn word_with_correct_letter(possible_word : String, rules_correct_letter: &Vec<rule::Rule>) -> bool{
    
    let mut valid : bool = true;

    for rule in rules_correct_letter {
        if valid && possible_word.chars().nth(rule.position).unwrap() != rule.letter{
            valid = false;
        }
    }

    valid
}

fn exclude_words_with_rules(possible_words: Vec<String>, rules_missing_letters: &Vec<rule::Rule>) -> Vec<String>{
    possible_words.into_iter()
                  .filter(|word| !word_with_rules(unidecode(word), rules_missing_letters))
                  .collect()
}

fn fix_words_correct(possible_words: Vec<String>, rules_correct_letters: &Vec<rule::Rule>) -> Vec<String>{
    possible_words.into_iter()
                  .filter(|word| word_with_correct_letter(unidecode(word), rules_correct_letters))
                  .collect()
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct CharsInString {
    letter: char,
    quantity: usize
}

fn count_letters(word: String) -> Vec<CharsInString>{
    
    let mut chars_in_string: Vec<CharsInString> = word.chars()
                                    .into_iter()
                                    .map(|letter| CharsInString { letter: letter, 
                                                                  quantity: word.matches(letter).count()
                                                                })
                                    .collect();

    chars_in_string.sort();
    chars_in_string.dedup();

    chars_in_string
}


fn main() {

    /*
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

    let json_to_parse_value = matches.value_of("json")
            .expect("Json with the ruleset is required");

    println!("JSON to parse {}", json_to_parse_value);*/

    
    let json_with_rules = r#"
    {
        "rules": 
        [
            {"letter": "a", "rule_type": "missing", "position": 0},
            {"letter": "i", "rule_type": "missing", "position": 0},
            {"letter": "g", "rule_type": "missing", "position": 0},
            {"letter": "p", "rule_type": "missing", "position": 0},
            {"letter": "b", "rule_type": "missing", "position": 0},
            {"letter": "r", "rule_type": "missing", "position": 0},
            {"letter": "m", "rule_type": "missing", "position": 0},
            {"letter": "s", "rule_type": "missing", "position": 0},
            {"letter": "f", "rule_type": "missing", "position": 0},
            {"letter": "u", "rule_type": "correct", "position": 1},
            {"letter": "d", "rule_type": "correct", "position": 0},
            {"letter": "l", "rule_type": "correct", "position": 3},
            {"letter": "o", "rule_type": "correct", "position": 4}
        ]
    }"#;

    let ruleset: rule::Ruleset = match rule::parse(json_with_rules) {
        Ok(result) => result,
        Err(error) => panic!("Error: {:?}", error),
    };

    let jaca = ruleset.rules.into_iter();
    
    let rules_missing : Vec<rule::Rule> = jaca.clone().filter(|rule| rule.rule_type == "missing").collect();
    let rules_correct : Vec<rule::Rule> = jaca.clone().filter(|rule| rule.rule_type == "correct").collect();

    let words_with_5_letters : Vec<String> = init_dictionary(false);

    let words_filtered = exclude_words_with_rules(words_with_5_letters, 
                                                  &rules_missing);

    let words_filtered2 = fix_words_correct(words_filtered, 
    &rules_correct);

    println!("Word {:?}", words_filtered2);
    println!("Count {:?}", words_filtered2.into_iter().count());


/*
    let xablau : String = String::from("maximo");
    let xabirou : String = String::from("medio");


    println!("{}", xablau.matches('m').count());

    
    let xibilou : Vec<String> = vec![xablau, xabirou];
   

    //println!("Aqui {:?}", xibilou);*/

   // let words_with_5_letters : Vec<String> = init_dictionary(false);

    //let x: Vec<Vec<CharsInString>> = words_with_5_letters.into_iter().map(|word| count_letters(word)).collect();

    //println!("{:?}", x);


}
