fn main() {
    // # Slices!
    let s = String::from("Hello there"); // 
    let first_word = first_word(&s); // immutable reference
    s.clear(); // mutable reference
    println!("{}", first_word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    return &s[..s.len()]
}
