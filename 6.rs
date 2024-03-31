// Implement a function that finds the longest common prefix of a given set of strings.

fn longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let first_str = &strs[0];
    let mut prefix = String::new();

    'outer: for (i, c) in first_str.chars().enumerate() {
        for other_str in strs.iter().skip(1) {
            if let Some(other_char) = other_str.chars().nth(i) {
                if other_char != c {
                    break 'outer;
                }
            } else {
                break 'outer;
            }
        }
        prefix.push(c);
    }

    prefix
}

fn main() {
    let strings = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];

    let common_prefix = longest_common_prefix(&strings);
    println!("Longest common prefix: {}", common_prefix);
}
