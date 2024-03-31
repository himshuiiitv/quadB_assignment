// checking palindrome
use std::io;

fn is_palindrome(input: &str) -> bool {
    let input = input.to_lowercase();
    let reversed = input.chars().rev().collect::<String>();
    input == reversed
}

fn main() {
    println!("Enter a string to check if it's a palindrome:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();
    
    let result = is_palindrome(input);
    
    println!("Is '{}' a palindrome? {}", input, result);
}
