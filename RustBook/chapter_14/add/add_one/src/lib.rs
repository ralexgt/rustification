use rand;

pub fn add_one(x: i32) -> i32 {
    let y: u8 = rand::random();
    x + y as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_ne!(3, add_one(3));
    }
}