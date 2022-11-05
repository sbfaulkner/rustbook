fn main() {
    {
        println!("===== part 1 =====");
        let s = "hello";
        println!("{}", s);
    }
    // <- s is out of scope here

    println!("===== part 2 =====");
    let s = String::from("hello");
    // s is immutable, so we can't modify it
    // s.push_str(", world!");
    println!("{}", s);

    println!("===== part 3 =====");
    let mut s = String::from("hello");
    // s is mutable, so we can modify it
    s.push_str(", world!");
    println!("{}", s);

    println!("===== part 4 =====");
    #[allow(unused_mut)]
    let mut s = String::from("hello");
    let s2 = &s;
    // s is not mutable because we have a reference to it
    // s.push_str(", world!");
    println!("{}", s);
    println!("{}", s2);

    println!("===== part 5 =====");
    let s1 = String::from("hello");
    let s2 = s1;
    // s1 is moved to s2, so we can't use s1 anymore
    // println!("{}", s1);
    println!("{}", s2);

    println!("===== part 6 =====");
    let s1 = String::from("hello");
    let s2 = s1.clone();
    // s1 is cloned to s2, so we can use s1
    println!("{}", s1);
    println!("{}", s2);

    println!("===== part 7 =====");
    let s = String::from("hello");
    takes_ownership(s);
    // s is moved by function call, so we can't use s anymore
    // println!("{}", s);
    let x = 5;
    makes_copy(x);
    // x is copied by function call, so we can use x
    println!("{}", x);

    println!("===== part 8 =====");
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    // s1 is moved on return, so we're able to use it
    println!("{}", s1);
    // s2 is moved by function call, so we can't use s2 anymore
    // println!("{}", s2);
    // but s3 is moved back, so we can use it
    println!("{}", s3);

    println!("===== part 9 =====");
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);

    println!("===== part 10 =====");
    let s1 = String::from("hello");
    let len = calculate_length_with_borrow(&s1);
    println!("The length of '{}' is {}.", s1, len);

    println!("===== part 11 =====");
    let mut s = String::from("hello");
    change(&s);

    println!("===== part 12 =====");
    change_mutable(& mut s);
    println!("{}", s);

    println!("===== part 13 =====");
    let mut s = String::from("hello");
    #[allow(unused_variables)]
    let r1 = &mut s;
    // r1 is mutable, so we can't have another mutable reference to s
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    println!("===== part 14 =====");
    let mut s = String::from("hello");
    {
        #[allow(unused_variables)]
        let r1 = &mut s;
    }
    // this is ok, because r1 is out of scope
    #[allow(unused_variables)]
    let r2 = &mut s;

    println!("===== part 15 =====");
    #[allow(unused_mut)]
    let mut s = String::from("hello");
    #[allow(unused_variables)]
    let r1 = &s;
    // this is ok, because r1 is immutable
    #[allow(unused_variables)]
    let r2 = &s;
    // this is not ok, because there are already immutable references to s
    // let r3 = &mut s;
    // println!("{}, {}, {}", r1, r2, r3);

    println!("===== part 16 =====");
    // let reference_to_nothing = dangle();
    #[allow(unused_variables)]
    let reference_to_something = &no_dangle();
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(x: i32) {
    println!("{}", x);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_with_borrow(s: &String) -> usize {
    s.len()
}

#[allow(unused_variables)]
fn change(some_string: &String) {
    // cannot modify (immutably) borrowed value
    // some_string.push_str(", world!");
}

fn change_mutable(some_string: &mut String) {
    some_string.push_str(", world!");
}

// this won't work because it returns a reference to a String (on the stack) that will be dropped
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
