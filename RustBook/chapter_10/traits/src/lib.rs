use core::fmt::{Display, Debug};

pub trait Summary {
    fn summarize(&self) -> String;
    fn summarize_with_default_behaviour(&self) -> String {
        String::from("This method on the trait is implemented by default and so you can override it or use the default implementation without specifing it in the struct impl")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub location: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} | written by {} | from {}", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
// impl Display for Tweet {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         todo!()
//     }
// } // implement Display for Tweet so we can implement ToString trait for any type that has the Display trait (it must be local)

pub fn notify(item: &impl Summary) { // this is syntax sugar for trait bounds
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_trait_bound<T: Summary>(item: &T) { // same as notify but using trait bound
    println!("Breaking news! {}", item.summarize());
}

pub fn more_traits<T: Summary + Display>(item: &T) {
    println!("{}", item);
}

pub fn where_clause<T, U>(item1: &T, item2: &U)
where
    T: Display + Summary,
    U: Display + Debug {
    todo!("{} {}", item1, item2);
}

pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// pub fn returns_summarizable_t<T: Summary>() -> T {
//     Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from(
//             "of course, as you probably already know, people",
//         ),
//         reply: false,
//         retweet: false,
//     }
// } // this doesn't work

pub struct Pair<T> {
    pub x: T,
    pub y: T,
}

impl<T> Pair<T> {
    fn _new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn _cmp_display(&self) {
        if self.x >= self.y {
            println!("Largest number is x: {}", self.x);
        } else {
            println!("Largest number is y: {}", self. y);
        }
    }
}

// impl<T: Display> ToString for T {
//     fn to_string(&self) -> String {
//         todo!()
//     }
// } // implement the ToString trait to any type that implements the Display trait (blanket implementation)

