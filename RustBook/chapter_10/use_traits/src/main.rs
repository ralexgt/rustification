use traits::{notify, notify_trait_bound, returns_summarizable, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("snowye"),
        content: String::from("Winter is finally here!"),
        reply: false,
        retweet: false,
    };

    println!("! New tweet ! {}", tweet.summarize());
    println!("Default summary: {} ", tweet.summarize_with_default_behaviour());

    notify(&tweet);
    notify_trait_bound(&tweet);
    
    println!("{}", returns_summarizable().summarize());
}