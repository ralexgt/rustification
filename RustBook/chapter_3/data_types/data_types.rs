/*
        SCALAR DATA TYPES
      i8    u8       8-bit
      i16   u16     16-bit
      i32   u32     32-bit
      i64   u64     64-bit
     i128  u128    128-bit
    isize usize arch-based
*/

/*
        NUMBER LITERALS
    decimal: 98_222 = 98222
    hex: 0xff
    octal: 0o77
    binary: 0b1111_0000
    bytes(u8 only): b'A' 
*/

// integers default to i32
// isize / usize for when indexing some sort of collection

// intiger overflow => panic in debug
//                  => goes to lowest value and increments from there on release

/*
        FLOATING POINT NUMBERS
    f32 - 32-bit
    f64 - 64-bit (default)

    All floating point numbers are signed
*/

/*
        BOOLEANS
    bool - 1 byte
    true
    false 
    used in if
*/

/*
        CHARS
    char - 4 bytes
    Unicode Scalar Value
    most primitive alphabetic type
    single quotes

*/

//          COMPOUND TYPES

/*
        TUPLES
    fixed length
    variety of types
    let tup: (i32, f64, u8) = (500, 6.4, 1); constructing a tuple
    let (x, y, z) = tup; destructing a tuple
    tup.0 (first element of the tuple)
    () empty tuple = is called unit
    Expressions implicitly return the unit value if they don't return any other value
*/

/*
        ARRAYS
    all elements must be of the same type
    fixed length
    let a = [1, 2, 3, 4, 5];
    useful to allocate data on the stack, not on the heap
    useful to have a fixed number of elements
    if you don't know if you should use a vector or an array, most probably a vector
    vector is similar but it can shrink or grow in size and is more flexible
    vector is part of the standard library
    prefer arrays when you know for sure the number of elements is fixed
    for example the months of the year
    let a:[type, number_of_elements] = [elements separated by comma]
    let a = [3; 5]; <=> a = [3, 3, 3, 3, 3]
    an array is a single chunk of memory of a known fixed size allocated on stack
    access elements of an array using indexing starting from 0
    let first = a[0];
    let second = a[1];
    reaching an index out of bounds results in a panic
*/
