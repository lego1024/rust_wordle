use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let filename="Lexique383.tsv";
    let mut i= 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(current_line) = line {
                let current_word = current_line.split('\t').next().unwrap_or("");
                if current_word.chars().count() == 5 {
                    println!("{}->{}", i, current_word);

                }
            }

            i+=1;
        
        }
    }

}