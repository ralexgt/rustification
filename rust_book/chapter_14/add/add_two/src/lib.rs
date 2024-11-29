use rand;

pub fn add_two(x: i32) -> i32 {
    let y: u8 = rand::random();
    x + y as i32 + y as i32
}