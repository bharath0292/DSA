fn _string_basics() {
    // String types
    let _str1: &str = "Hello"; // string slice
    let _string1 = String::from("Hello"); // owned string

    // String creation
    let mut s = String::new();
    s.push_str("Hello");

    // Concatenation
    let s1 = String::from("Hello");
    let s2 = String::from(" World");
    let _s3 = s1 + &s2; // s1 moved here

    // String with capacity
    let mut s = String::with_capacity(10);
    s.push_str("Hello");

    // Iterating over characters
    for c in "Hello".chars() {
        println!("{}", c)
    }

    // Iterating over bytes
    for b in "Hello".bytes() {
        println!("{}", b)
    }
}

fn _string_operations() {
    let s = String::from("Hello World");

    // Substring
    let _hello = &s[0..5];

    // Split
    let _parts: Vec<&str> = s.split(",").collect();

    // Join
    let _joined = vec!["Hello", "World"].join(",");

    // Contains, Starts, Ends
    let _contains = s.contains("World");
    let _starts_with = s.starts_with("Hello");
    let _ends_with = s.ends_with("!");

    // Case conversion
    let _upper = s.to_uppercase();
    let _lower = s.to_lowercase();

    // Trim
    let _trimmed = s.trim();

    // Replace
    let _replaced = s.replace("World", "Rust");
}
