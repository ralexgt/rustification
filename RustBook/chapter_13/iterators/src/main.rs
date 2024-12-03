fn main() {
    let v1 = vec![1, 3, 4];
    let v1_iter = v1.iter(); // into_iter() returns an iterator that has ownership over its value | iter_mut() returns an iterator over mutable values
    // iterators are lazy. This snippet in itself does nothing. The iterator needs methods to consume.
    
    for item in v1_iter {
        println!("Got: {}", item);
    }
}
