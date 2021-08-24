fn main() {
    let str = String::from("Hello, world!");
    let res = first_word(&str);
    // str.clear();
    println!("res: {}", res);
    let my_str = "hi cinob!";
    let res = first_word(my_str);
    println!("res: {}", res);
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