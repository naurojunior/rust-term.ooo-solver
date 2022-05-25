use std::path::Path;

mod dictionary;
mod rule;

use unidecode::unidecode;
use std::fs::File;
use std::io::Read;

//use clap::{Arg, Command};

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
        if valid && possible_word.chars().nth(rule.position.unwrap_or_default()).unwrap() != rule.letter{
            valid = false;
        }
    }

    valid
}

fn exclude_words_with_rules(possible_words: Vec<String>, rules_missing_letters: &Vec<rule::Rule>) -> Vec<String>{

    if rules_missing_letters.len() == 0 {
        return possible_words;
    }

    possible_words.into_iter()
                  .filter(|word| !word_with_rules(unidecode(word), rules_missing_letters))
                  .collect()
}

fn word_with_incorrect_position_letter(possible_word : String, rules_incorrect_position_letter: &Vec<rule::Rule>) -> bool{
    
    let mut valid : bool = true;

    for rule in rules_incorrect_position_letter {
        if valid && possible_word.chars().nth(rule.position.unwrap_or_default()).unwrap() == rule.letter{
            valid = false;
        }
    }

    valid
}

fn word_contains_all_letters(possible_word : String, rules_incorrect_position_letter: &Vec<rule::Rule>) -> bool {
    let mut valid : bool = true;

    for rule in rules_incorrect_position_letter {
        if valid && possible_word.chars().any(|x| x == rule.letter){
            valid = true
        }else{
            valid = false
        }
    }

    valid

}


fn words_with_letter_in_correct_position(possible_words: Vec<String>, rules_correct_letters: &Vec<rule::Rule>) -> Vec<String>{

    if rules_correct_letters.len() == 0 {
        return possible_words;
    }

    possible_words.into_iter()
                  .filter(|word| word_with_correct_letter(unidecode(word), rules_correct_letters))
                  .collect()
}

fn words_with_letter_in_incorrect_position(possible_words: Vec<String>, rules_incorrect_position: &Vec<rule::Rule>) -> Vec<String>{

    if rules_incorrect_position.len() == 0 {
        return possible_words;
    }

    possible_words.into_iter()
                  .filter(|word| word_contains_all_letters(unidecode(word),rules_incorrect_position))
                  .filter(|word| word_with_incorrect_position_letter(unidecode(word), rules_incorrect_position))
                  .collect()
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct CharsInString {
    letter: char,
    quantity: usize
}

/*
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
}*/


fn main() {

    match File::open("rules.txt") {
        Ok(mut file) => {
            let mut content = String::new();    
            file.read_to_string(&mut content).unwrap();

            let ruleset: rule::Ruleset = match rule::parse(content) {
                Ok(result) => result,
                Err(error) => panic!("Error: {:?}", error),
            };
                    
            let ruleset_iter = ruleset.rules.into_iter();
            
            let rules_missing : Vec<rule::Rule> = ruleset_iter.clone().filter(|rule| rule.rule_type == "wrong").collect();
            let rules_correct : Vec<rule::Rule> = ruleset_iter.clone().filter(|rule| rule.rule_type == "right").collect();
            let rules_incorrect_position : Vec<rule::Rule> = ruleset_iter.clone().filter(|rule| rule.rule_type == "place").collect();

            let words_with_5_letters : Vec<String> = init_dictionary(false);

            let words_filtered = exclude_words_with_rules(words_with_5_letters, &rules_missing);

            let words_filtered2 = words_with_letter_in_correct_position(words_filtered, &rules_correct);

            let words_filtered3 = words_with_letter_in_incorrect_position(words_filtered2, &rules_incorrect_position);

            println!("Word {:?}", words_filtered3);
            println!("Count {:?}", words_filtered3.into_iter().count());


        },
        Err(error) => {
            println!("Error opening file {}: {}", "rules.txt", error);
        },
    }
}
