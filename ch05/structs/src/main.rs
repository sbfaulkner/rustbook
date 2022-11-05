struct User {
    username: String,
    email: String,
    #[allow(dead_code)]
    sign_in_count: u64,
    #[allow(dead_code)]
    active: bool,
}

fn main() {
    let user = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}, {}, {}, {}", user.email, user.username, user.active, user.sign_in_count);

    let mut user = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}, {}, {}, {}", user.email, user.username, user.active, user.sign_in_count);

    user.email = String::from("anotheremail@example.com");

    println!("{}, {}, {}, {}", user.email, user.username, user.active, user.sign_in_count);

    let user = build_user(String::from("someone@example.com"), String::from("someusername123"));

    println!("{}, {}, {}, {}", user.email, user.username, user.active, user.sign_in_count);

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user
    };

    println!("{}, {}, {}, {}", user2.email, user2.username, user2.active, user2.sign_in_count);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    #[allow(unused_variables)]
    let black = Color(0, 0, 0);
    #[allow(unused_variables)]
    let origin = Point(0, 0, 0);

    println!("black: ({}, {}, {})", black.0, black.1, black.2);
    println!("origin: ({}, {}, {})", origin.0, origin.1, origin.2);

    // won't work because they are different types (even though they have the same fields)
    // let mut color:Color = origin;

    let Color(r, g, b) = black;
    let Point(x, y, z) = origin;

    println!("black: ({}, {}, {})", r, g, b);
    println!("origin: ({}, {}, {})", x, y, z);

    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area_scalars(width1, height1)
    );

    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);

    println!(
        "sq is {:?} and is {} square pixels.",
        sq,
        sq.area()
    );
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area_scalars(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
