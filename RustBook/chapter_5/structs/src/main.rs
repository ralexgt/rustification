fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("newemail@example.com");

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        email: String::from("other@example.com"),
        ..user2
    };
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let subject = AlwaysEqual;

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!(
        "rect1 is {:#?}",
        rect1
    ); // println! takes a reference to the value
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale), // here we do not have to give a reference because we assign the value to width in the and the ownership is transfered
        height: 50,
    };
    dbg!(&rect2); // dbg! takes ownership and returns it, but we give it as a reference so we don't have to return it to the original variable
    
    let rect3 = rect2;
    println!("The area of rectangle 3 is {} square pixels", 
        rect3.area() // calling a method of the Rectangle type implemented in impl Rectangle {}
    );

    println!("rect3 has a nonzero width: {}. Its width is: {}",
        rect3.width(), rect3.width  
    );
    println!("rect1 can fit inside rect3: {}",
        rect1.can_hold(&rect3)
    );
}

struct User {
    username: String,
    active: bool,
    sign_in_count: u64,
    email: String,    
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

#[derive(Debug)] // outer atribute
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width >= rect.width && self.height >= rect.height
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
} // this is a function that takes as argument a reference to a Rectangle variable 