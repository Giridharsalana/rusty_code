// Rust Practice by Gs
use rand::prelude::*;

fn main() {
    println!("# Rust Practice by Gs #");
    let _key = "J2yVZrL1dJXSRwd8pUKfEw==Hczdw6PyL2NBhXqc";
    // println!("{:?}", letters("Giri".to_string()));
    // println!("{}", random_number());
}

fn _letters(word: String) -> Vec<char> {
    let char_vec: Vec<char> = word.chars().collect();
    char_vec
}

// function to generate random number
fn _random_number() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}
