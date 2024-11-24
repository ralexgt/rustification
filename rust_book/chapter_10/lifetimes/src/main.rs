use std::fmt::Display;

fn main() {
    // let r;
    // {
    //     let x = 5;
    //     r = &x; // r borrow the x value, which will be lost at the end of the scope
    // }
    // println!("r: {}", r); // x does not live long enough to be borrowed here
    
    let string1 = "abcd";
    let string2 = String::from("xyz");
    let result = longest(string1, string2.as_str());
    println!("The longest of {} and {} is: {}", string1, string2, result);

    let ipsum = String::from("Lorem. ipsum. ...");
    let first_sentence = ipsum.split('.').next().unwrap();
    let i = Lorem {
        text: first_sentence,
    };
    println!("{}", i.text);

    println!("The longest of {} and {} is: {}", string1, string2, longest_with_announcement(string1, string2.as_str(), ipsum));

    // one special lifetime annotation is 'static. It tells the compiler that the reference lives as long as the program runs
    // string literals (&str) have this annotation by default. Errors might suggest using it when we try to create a dangling
    // reference or mismatch available life times, but it is most often than not our mistake and we should think if the
    // reference really needs the 'static annotation and if it really lives for the entire program.
}

struct Lorem<'a> {
    text: &'a str,
}
impl<'a> Lorem<'a> {
    fn _level(&self) -> i32 {
        1
    }
    fn _announce_and_return_text(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.text
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // the returned &str will be available as long as bot x and y are available
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T, ) -> &'a str
where
    T: Display, {
        println!("Announcement: {} ", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }