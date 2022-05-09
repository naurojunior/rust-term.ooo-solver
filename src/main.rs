use std::env;
use std::path::Path;

mod dictionary;

fn should_generate_dictionary(force_generate_dictionary: bool, output_file_exists: bool) -> bool{
    force_generate_dictionary || !output_file_exists
}

fn force_generate_dictionary(args: Vec<String>) -> bool{
    for argument in args {
        if argument == "--force-generate-dictionary" || argument == "-f" {
            return true;
        }
    }
    
    false
}

fn output_file_exists(output_file_name: &str) -> bool{
    Path::new(output_file_name).exists()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let output_file_name = "words-5-letters.txt";

    if should_generate_dictionary(force_generate_dictionary(args), output_file_exists(output_file_name)) {
        println!("{}", "Dictionary generated");
        dictionary::generate("words.txt", output_file_name)
    }else{
        println!("{}", "Using already generated dictionary");
    }
}
