
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::prelude::*;

pub fn generate(input_file_name: &str, output_file_name: &str) {
    
    let words_with_five_letters: Vec<String> = fetch_words_with_5_letters(input_file_name);

    let _ = write_final_dictionary(words_with_five_letters, output_file_name);
}


fn write_final_dictionary(words_with_five_letters: Vec<String>, output_file_name: &str) -> std::io::Result<()>{
    
    let mut file = File::create(output_file_name)?;

    for word in words_with_five_letters {
        file.write_all(word.as_bytes())?;
        file.write(b"\r\n")?;    
    }
    
    Ok(())
}

fn fetch_words_with_5_letters(input_file_name: &str) -> Vec<String>{

    let mut words_with_five_letters: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(input_file_name) {
        for line in lines {
            if let Ok(word) = line {
                if word.chars().count() == 5 {
                    words_with_five_letters.push(word.to_string());
                }
            }
        }
    }

   words_with_five_letters
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}