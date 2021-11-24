struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let me = build_user("chin".to_string(), "some mail".to_string());
    let someone = User {
        username: "anon".to_string(),
        email: "mail".to_string(),
        ..me // similar to js syntax where we can destructure into another struct
    };
}

fn build_user(username: String, email: String) -> User {
    User {
        // field init shorthand
        username,
        email,
        sign_in_count: 1,
        active: false,
    }
}

#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

// Note the use of the reference here
// This is to avoid owning the data - we want it to be valid in the main program instead of shifting ownership into the function
fn area(r: &Rect) -> u32 {
    r.width * r.height
}

fn rect() {
    let r = Rect {
        width: 32,
        height: 32,
    };
    let a = area(&r);
    println!("r: {:#?}, area: {}", r, a);
    let a = r.area();
    println!("area: {}", a)
}

// methods for structs
impl Rect {
    // note the use of &self syntax
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: Rect) -> bool {
        self.height >= other.height && self.width >= other.width
    }

    // this is not a method but a normal function
    // this will be namespaced by Rect and can be called as follows:
    // Rect::square(2)
    fn square(side: u32) -> Rect {
        Rect {
            width: side,
            height: side,
        }
    }
}
