fn main() {
    // let mut s = String::from("hello");
    // println!("{s}");
    // s.push_str(", world!");
    // println!("{s}");
    let x = 5;
    let y = x; // scalar values implement the Copy trait, they have fixed size at compile time and are stored on stack. We do not need to worry about cloning the values because the expense in negligible
    println!("x={x}\ny={y}");
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved in s2 and no longer valid
    // println!("{s1}");
    println!("{s2}");
    let s3 = s2.clone(); // s2 is cloned sa the s2 declared on line 10 is not moved. Data on the heap is cloned which can get very expensive in terms of performance 
    println!("{s3}");


    let s = String::from("hello");  // s comes into scope

    takes_ownership(s); // s's value moves into the function...
    // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
    // but i32 is Copy, so it's okay to still
    // use x afterward
   
    let /* mut */ s = String::from("hello");
    // s = takes_and_gives_back(s);
    // s.push_str("aa");
    // s = takes_and_gives_back(s);
    let s2 = takes_and_gives_back(s);
    println!("{s2}");

    let (mut s2, len) = calculate_length(s2);
    println!("the length of '{s2}' is {len}.");

    let len = calculate_length_without_ownership(&mut s2);
    println!("{len}");

    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; // you can't have 2 mutable references to the same value: both would be able to mutate it and it prones errors
    println!("{}", r1, /* r2 */);
    let r2 = &mut s; // we can do this now because r1 finished using the reference
    println!("{}", r2);
    // println!("{}", r1); // this would give an error on line 45 because r1 would be still in scope 
    // so multiple imutable references are allowed, but you can't have a mutable reference and other imutable ones or more than one mutable reference
    // the scope of a reference ends when the reference is last time used or when the block ends

    // try creating a dangling reference (reference to data that was deallocated)
    // let reference_to_nothing = dangle();

    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5
    println!("{}", word);
    s.clear(); // this empties the String, making it equal to ""
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
    println!("{}", word);
    let word = first_word(&s);
    println!("{}", word);

    // string slices
    let s = String::from("Hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}\n{}", hello, world);
    let /* mut */ s = String::from("Hello word");
    let word = first_word_with_slices(&s);
    println!("{}", word);
    // s.clear();
    // println!("{}", word);

    // other slices (ex array slices of i32)
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope
    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length) // return multiple values by returning a tuple
}

fn calculate_length_without_ownership(s: &mut String) -> usize {
    s.push_str("a");
    s.len()
}

// fn dangle() -> &String {
//     let s = String::from("hello world");

//     &s
// } // example for a dangling reference

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_with_slices(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}