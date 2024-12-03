use std::collections::HashMap;
fn main() {
    // let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(4);
    v.push(5);
    v.push(6);
    v.push(7);

    let fourth = &v[3];
    println!("The forth element in v is: {fourth}");
    
    let tenth: Option<&i32> = v.get(11);
    match tenth {
        Some(tenth) => println!("The tenth element is {tenth}"),
        None => println!("There is no tenth element."),
    }

    // let does_not_exist = &v[100]; index out of bounds <=> it references to a non-existing value -> it will panic
    // println!("{does_not_exist}"); 
    let does_not_exist = v.get(100);  // we get Some(&i32) or None and we have to handle each case 
    println!("{does_not_exist:?}");

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    println!("The first element is: {first}");
    v.push(6);

    let mut v = vec![73, 24, 15];
    for i in &mut v {
        *i = *i * 2;
        println!("{}", i);
    }
    for (i, item) in v.iter().enumerate() {
        println!("{}: {}", i, item);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for (i, item) in row.iter().enumerate() {
        match item {
            &SpreadsheetCell::Int(value) => println!("{i}: {value}"),
            &SpreadsheetCell::Float(value) => println!("{i}: {value}"),
            SpreadsheetCell::Text(value) => println!("{i}: {value}"),
        }
    }
    //

    let s = String::from("hello"); // same thing
    let s = "hello".to_string();   // same thing
    let hello = String::from("नमस्ते"); // strings are utf-8 encoded
    // use + or format! macro to concatenate string values
    let mut s = String::from("foo");
    s.push_str("bar"); // append to the string s (s must be mutable)
    let mut s1 = String::from("foo");
    let s2 = "bar";
    println!("s1 is {s1}");
    s1.push_str(s2); // push_str takes a string literal, meaning it doesn't take ownership. We would want to use s2 even after appending its contents to s1
    println!("s2 is {s2}");
    println!("s1 becomes {s1}");
    let s = format!("{s}-{hello}");
    println!("{s}");
    // let char = s[0]; // you can't index into strings. You can use slices, but it's risky and can crash the program. Iterate through characters or bytes as followed
    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }
    // 


    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let blue_team = String::from("Blue");
    let blue_score = scores.get(&blue_team).copied().unwrap_or(0); // get the value of a particular key in a hashmap
    println!("Team {blue_team} scored {blue_score} points");
    for (key, value) in &scores {
        println!("{key}: {value}");
    } // iterate over the hashmap

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // the hashmap takes ownership of its value
    // if we give it by reference then the hashmap won't take ownership, but the reference must be valid for at least as long as the hashmap is valid
    
    let mut scores = HashMap::new();

    scores.insert(String::from("Green"), 12);
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");
    // insert overwrites the value if the key already exists in the hashmap

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);   // .entry(key) returns an enum Entry value with the value already existing at the key or null, if null or_insert(value) inserts the value with the respective key

    println!("{scores:?}");
    // insert keeps the existing value or adds a pair {key: value} if the key doesn't already exist

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() { // splits the text by whitespaces and word gets each section iterating over them
        let count = map.entry(word).or_insert(0); // if the word already appeared keep the value and increment on the next line | otherwise set it to 0 (it appeared once)
        *count += 1;
    }

    println!("{map:?}");
    // update the value at a specific key keeping account of the old value
    // in this specific case we add a count = 0 if it first appears and then we increment by one each time the word appears in the string

    // exercises
    let mut v = vec![73, 28, 32, 16, 28, 16, 102, 12, 16, 13, 16, 17, 5];
    v.sort();
    println!("{v:?}");
    let median_v = &v[v.len() / 2];
    println!("{median_v} {}", v.len() / 2);

    let mut occurencies= HashMap::new();
    for num in &v {
        let count = occurencies.entry(*num).or_insert(0);
        *count += 1;
    }
    let mut max_occ = occurencies.get(&v[0]).copied().unwrap_or(0);
    let mut mode = v[0];
    for (num, occ) in &occurencies {
        if *occ >  max_occ {
            max_occ = *occ;
            mode = *num;
        }
    }
    println!("{:?}", &occurencies);
    println!("The mode of the vector is: {mode}.");

    // ex2
    let mut s = String::from("vapple");
    to_pig_latin(&mut s);
    println!("{s}");


}  

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn to_pig_latin(s: &mut String) {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let first_letter = &s.to_lowercase().chars().nth(0).unwrap();
    if vowels.contains(&first_letter) {
        s.push_str("-hay");
    } else {
        s.remove(0);
        s.push('-');
        s.push(*first_letter);
        s.push_str("ay");
    }
}