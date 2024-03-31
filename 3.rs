// Given a string of words, implement a function that returns the shortest word in the string.

fn shortest_word(input: &str) -> Option<&str> {
    input.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
    let sentence = "The quick brown fox jumps over the lazy dog";
    match shortest_word(sentence) {
        Some(shortest) => println!("The shortest word is: {}", shortest),
        None => println!("No words found in the string"),
    }
}
