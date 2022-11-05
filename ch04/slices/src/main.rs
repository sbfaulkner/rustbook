fn main() {
    let mut s = String::from("hello world");

    let word = first_word_length(&s);
    // word has the value 5
    println!("{}", word);

    s.clear();

    // word still has the value 5 - despite the string being empty (ie. potential bug)
    println!("{}", word);

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} {}", hello, world);

    // first and last character are optional/assumed in the range
    let hello = &s[..5];
    let world = &s[6..];

    println!("{} {}", hello, world);

    #[allow(unused_mut)]
    let mut s = String::from("hello world");

    // using string slice instead...
    let word = first_word(&s);

    println!("{}", word);

    // can't clear because of the reference (by the slice) to s
    // s.clear();

    println!("{}", word);

    // this doesn't work because &str isn't &String
    // let s = "hello world";
    // let word = first_word(s);

    let s1 = String::from("hello world");
    let s2 = "hello world";

    // improved version of first_word works because it takes a string slice
    let word = first_word_improved(&s1);
    println!("{}", word);

    let word = first_word_improved(s2);
    println!("{}", word);

}

fn first_word_length(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn first_word_improved(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
