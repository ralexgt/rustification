fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    println!("Largest of {:?} is: {}", number_list, largest(&number_list));
    let number_list = vec![7, 12, 72, 15, 17, 34, 62, 17];
    println!("Largest of {:?} is: {}", number_list, largest(&number_list));
    let char_list = vec!['a', 'c', 'f', 'x', 'r'];
    println!("Largest char of {:?} is: {}", char_list, largest(&char_list));

    // let invalid_point = Point {x: 32, y: 4.7};  // mismatched types => in the definition of Point we said both values will have the same T value
    let valid_point = Point {x: 7, _y: 9};
    let _valid_ver_point = _VersitilePoint {_x: 3, _y: 12.5};
    let _valid_ver_point = _VersitilePoint {_x: 2, _y: 2};
    let _valid_ver_point = _VersitilePoint {_x: 8.5, _y: 13.7};

    println!("valid_point.x = {}", valid_point.x());
}

// generics in structs
struct Point<T> {
    x: T,
    _y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {
    fn _distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self._y.powi(2)).sqrt()
    }
} // we can also implement functions for when the generic takes specific values
struct _VersitilePoint<T, U> {
    _x: T,
    _y: U,
}
impl<T1, U1> _VersitilePoint<T1, U1> {
    fn _mixup<T2, U2>(self, other: _VersitilePoint<T2, U2>) -> _VersitilePoint<T1, U2> {
        _VersitilePoint {
            _x: self._x,
            _y: other._y,
        }
    }
}

// generics on enums
enum _Option<T> {
    Some(T),
    None,
}
enum _Result<T,E> {
    Ok(T),
    Err(E),
}


// generics in functions
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for i in list.iter() {
        if i > largest {
            largest = i;
        }
    }
    largest
}