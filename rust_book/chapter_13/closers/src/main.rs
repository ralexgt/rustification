use std::thread;
use std::time::Duration;

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue,ShirtColor::Red, ShirtColor::Blue, ShirtColor::Blue]
    };
    let user1_pref = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user1_pref);
    println!("The user with preference {:?} gets {:?}", user1_pref, giveaway1);
     
    let user2_pref = None;
    let giveaway2 = store.giveaway(user2_pref);
    println!("The user with preference {:?} gets {:?}", user2_pref, giveaway2);


    let _expensive_closure = |num: i32| -> i32 {
        println!("Artificially slow it down");
        thread::sleep(Duration::from_secs(2));
        num + 1
    };

    fn _add_one_v1(x: i32) -> i32 {
        x + 1
    } // function
    let _add_one_v2 = |x: i32| -> i32 {
        x + 1
    }; // verbouse closure
    let add_one_v3 = |x| {
        x + 1
    };
    let add_one_v4 = |x| x + 1.;

    let _n = add_one_v3(3);
    let _n = add_one_v4(1.2);
    //

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    let only_borrows = || println!("From closure: {list:?}");
    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
    //
    let mut borrows_mutably = || list.push(4);
    borrows_mutably();
    println!("After calling closure: {list:?}");
    // only_borrows(); // this will give an error on line 45: we are borrowing list as mutable and then here borrowing it as immutable again


    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    // println!("{:?}", list); borrow of moved value (move from closure at line 54)
    

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];
    println!("{list:#?}");
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}
impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stock())
    }

    fn most_stock(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}