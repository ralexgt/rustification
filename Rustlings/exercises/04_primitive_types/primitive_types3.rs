fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    
    let mut a: [i32; 100] = [42; 100];
    for i in &mut a {
        *i+=1;
        print!("{} ", i);
    }
    for i in a {
        print!("{} ", i);
    }
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
