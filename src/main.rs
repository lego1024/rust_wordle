use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

use unidecode::unidecode;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn stats(words: & std::collections::HashSet<String>) {
    let mut freq:[[u32;26];5] = [[0;26];5];
    for word in words {
        println!("{}", word);
        for ci in word.char_indices() {
            let indice: usize = ci.0 as usize;
            let c:char = ci.1;
            let n:usize = c as usize - 0x61;
            freq[indice][n] += 1;
        }
    }
    for (_i, row) in freq.iter().enumerate() {
        for (_j, value) in row.iter().enumerate() {
            print!(" {number: >width$}" , number=value, width=4);
        }
        print!("\n");
    }
    

}
fn print_guess(guess: &str, expected: &str) {
    print!("{}:", guess);
    if guess.len() == expected.len() {
        for gic in guess.char_indices() {
            let char_pos_in_exptected = expected.find(gic.1).unwrap_or(guess.len());
            if gic.0 == char_pos_in_exptected {
                print!("🟩");
            } else if char_pos_in_exptected == expected.len() {
                print!("🟥");
            } else {
                print!("🟨");
            }
        }
    }
    println!("");
}

fn main() {
    let filename = "Lexique383.tsv";
    let mut words_set = std::collections::HashSet::new();
    let mut sanitize_word;

    let word_size = 5;

    if let Ok(lines) = read_lines(filename) {
        let re = Regex::new(r"^[a-z]{5}$").unwrap();
        for line in lines {
            if let Ok(current_line) = line {
                let current_word = current_line.split('\t').next().unwrap_or("");
                sanitize_word = unidecode(current_word);
                if re.is_match(&sanitize_word) && sanitize_word.chars().count() == word_size {
                    words_set.insert(sanitize_word);
                }
            }
        }
        stats(&words_set);

        let secret_number = rand::thread_rng().gen_range(1..words_set.len());
        let mut secret_word = String::from("");
        let mut wi: usize = 0;
        for w in words_set.clone() {
            if wi == secret_number {
                secret_word = String::from(&w);
            }
            wi += 1;
        }
        println!("Secret word to found = {}", secret_word);
        let mut nb_try = 6;
        loop {
            println!("Please input your guess. (try#{})", nb_try);
            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            guess = guess.trim().to_lowercase();
            if guess.len() != word_size {
                println!("Bad size, try again.");
                continue;
            }
            if ! words_set.clone().contains(&guess) {
                println!("Unknown word, try again.");
                continue;
                
            }
            if secret_word.eq_ignore_ascii_case(&guess.clone()) {
                println!("You guessed: {}", secret_word);
                break;
            }
            println!("Wrong");
            print_guess(&guess, &secret_word);
            nb_try -= 1;
            if nb_try <= 0 {
                break;
            }
        }
    }
}
