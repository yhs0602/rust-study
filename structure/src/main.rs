struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
}

fn main() {
    let user1 = User {
         email: String::from("someone@example.com"),
         username: String::from("someusername123"),
         active: true,
         sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("soama@aaa.com"),
        username: String::from("akakaak"),
        ..user1
    };

    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    
    let rect1 = (50, 30);

    let rect2 = Rectangle { length: 50, width: 30 };
    println!("The area of the rectangle is {} square pixels.", area(rect1));

    println!("The rect2 is {:?}", rect2);
    println!("The area of rect2 is {}", rect2.area());

}

fn build_user(email: String, username: String) -> User {
    User {
        email, // : email,
        username, // : username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
