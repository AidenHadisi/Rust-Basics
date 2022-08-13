//checks if given string is plural or singular.
pub fn inspect(test: &String) {
    if test.ends_with("s") {
        println!("plural")
    } else {
        println!("singular")
    }
}

/// Adds "s" to given string if it doesn't have one already.
pub fn change(test: &mut String) {
    if !test.ends_with("s") {
        test.push_str("s");
    }
}

///checks if a string starts with "b" and also contains and "a"
pub fn eat(test: String) -> bool {
    test.starts_with("b") && test.contains("a")
}

pub fn add(a: &i32, b: &i32) -> i32 {
    *a + *b
}
