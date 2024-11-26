pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
pub fn add_two(i: u64) -> u64 {
    i + 2
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {value}."
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {value}."
            );
        }

        Guess { value }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[should_panic]
    #[test]
    fn make_it_panic() {
        panic!("Fail the test");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 7,
            height: 4,
        };
        let smaller = Rectangle {
            width: 5,
            height: 2,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn add_two_test() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
    
    #[test]
    fn is_anything_but_0() {
        let result = 3;
        assert_ne!(result, 0);
    }

    #[test]
    fn greeting_test() {
        let greeting = greeting("Andrew");
        assert!(
            greeting.contains("Andrew"),
            "Greeting did not contain the name, value was {}", greeting
        );
    }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test] // example of test without assert_eq, using Result
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
