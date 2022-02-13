use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::collections::HashSet;
use std::collections::HashMap;
use unidecode::unidecode;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn stats(words: & HashSet<String>,
    words_tried:& HashMap<String,Vec<i32>>) {
    let mut freq:[[u32;26];5] = [[0;26];5];
    for word in words {
        for ci in word.char_indices() {
            let indice: usize = ci.0 as usize;
            let c:char = ci.1;
            let n:usize = c as usize - 0x61;
            freq[indice][n] += 1;
        }
    }
    // for (_i, row) in freq.iter().enumerate() {
    //     for (_j, value) in row.iter().enumerate() {
    //         print!(" {number: >width$}" , number=value, width=4);
    //     }
    //     print!("\n");
    // }
    let mut e_word_max:f32 = 0.0;
    let mut word_max = String::from("");
    for w2 in words {
        let mut e_word = 0.0;
        for ci2 in w2.char_indices() {
            let indice: usize = ci2.0 as usize;
            let c:char = ci2.1;
            let n:usize = c as usize - 0x61;

            let mut px:f32 = freq[indice][n] as f32 / words.len() as f32;
            for t in words_tried {jjj
                if c == t.0.chars().nth(indice).unwrap_or_default() && t.1[indice] == 2 {
                    px = 0.0;
                }
            }
            e_word += px * (1.0 / px).log(2.0);
        }
        if e_word > e_word_max {
            e_word_max = e_word;
            word_max = w2.clone();
        }
    }
    println!("word max :{} {}", word_max, e_word_max);
    

}
fn print_guess(guess: &str, expected: &str) -> Vec<i32> {
    let mut result : Vec<i32> = vec![0,0,0,0,0];
    print!("{}:", guess);
    if guess.len() == expected.len() {
        for gic in guess.char_indices() {
            let char_pos_in_exptected = expected.find(gic.1).unwrap_or(guess.len());
            if gic.0 == char_pos_in_exptected {
                result[gic.0] = 0;
                print!("🟩");
            } else if char_pos_in_exptected == expected.len() {
                result[gic.0] = 2;
                print!("🟥");
            } else {
                result[gic.0] = 1;
                print!("🟨");
            }
        }
    }
    println!("");
    result
}

fn main() {
    let filename = "Lexique383.tsv";
    let mut words_set = std::collections::HashSet::new();
    let mut sanitize_word;

    let word_size = 5;
    let mut words_tried: HashMap<String, Vec<i32>> = std::collections::HashMap::new();

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
        let mut nb_try = 20;
        loop {
            stats(&words_set, &words_tried);
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
            let rep = print_guess(&guess, &secret_word);
            words_tried.insert(guess.clone(), rep);
            println!("Wrong");


            nb_try -= 1;
            if nb_try <= 0 {
                break;
            }
        }
    }
}
