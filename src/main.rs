use std::env;
use std::path::Path;

mod dictionary;
mod rule;

fn should_generate_dictionary(force_generate_dictionary: bool, output_file_exists: bool) -> bool{
    force_generate_dictionary || !output_file_exists
}

fn is_force_generate_dictionary(args: Vec<String>) -> bool{
    
    args.iter()
        .filter(|&argument| { argument == "--force-generate-dictionary" || argument == "-f" })
        .count() > 0

}

fn output_file_exists(output_file_name: &str) -> bool{
    Path::new(output_file_name).exists()
}

fn init_dictionary(args: Vec<String>) {
    let output_file_name = "words-5-letters.txt";

    if should_generate_dictionary(is_force_generate_dictionary(args), output_file_exists(output_file_name)) {
        println!("{}", "Dictionary generated");
        dictionary::generate("words.txt", output_file_name)
    }else{
        println!("{}", "Using already generated dictionary");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    init_dictionary(args);


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
    }"#;

    let _ = match rule::parse(json_with_rules) {
        Ok(result) => result,
        Err(error) => panic!("Error: {:?}", error),
    };

}
