fn main() {
    let s = String::from("hello world");
    let s2 = "hello world";

    let word: &str = first_word(&s);
    println!("{}", word);
    
    let word: &str = first_word(&s2);
    println!("{}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}