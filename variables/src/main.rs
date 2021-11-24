fn main() {
    let mut x = 5;
    println!("x: {}", x);
    x = 7;
    println!("x: {}", x);

    // type is &str -> it is a pointer (on the stack) to a string
    // Rust hardcodes this into the final executable and this means that it's fast + efficient
    let _string = "hello";

    // 2 steps when we use the String type
    // Step 1: we request memory from the os to hold the given string (in here, "hello")
    // Step 2: return memory back to the os when string is gc-ed/free-d
    let mut s = String::from("hello"); // Step 1
    s.push_str(", world");

    let s2 = s;
    // s.push_str("!"); // This is considered invalid because the string that s was originally pointing to has been `moved` to s2
    // println!("s1: {}", s);
    println!("s2: {}", s2);

    let mut cloned = s2.clone();
    cloned.push_str("!");
    println!("cloned: {}", cloned);
    println!("s2: {}", s2);

    let tup = (1, 2, 3);

    let (a, b, c) = tup;
    println!("destructuring: {}, {}, {}", a, b, c);

    println!("test: {}", test(1234));

    let y = {
        let x = 3;
        x + 1 // no semicolon - this is an expression and the return value of the {} block
    };
    println!("y: {}", y);

    // arrays are immutable
    let arr = [1, 2, 3, 4, 5];
    // 2 loop constructs, while and for
    let mut index = 0;
    while index < 5 {
        println!("arr item at index {}: {}", index, arr[index]);
        index += 1;
    }

    // note the iter() call
    for item in arr.iter() {
        println!("item: {}", item);
    }

    let hello = String::from("hello world");

    let first = first_word(&hello);
    println!("first word: {}", first);
    ownership()
} // Step 2: s goes out of scope -> lifetime is over and `drop` is called to return memory back to the os

fn test(x: i32) -> i32 {
    x
}

fn ownership() {
    let mut s = String::from("allo");
    borrow_ownership(&mut s); // move occurs from s into the function `take_ownership`
    println!("{}", s);
}

fn take_ownership(s: String) -> String {
    println!("taking ownership of {}", s);
    s
}

fn borrow_ownership(s: &mut String) {
    s.push_str("added this thing")
}

fn mutability() {
    let mut s = String::from("alo");
    let r = &s;
    // s.push_str("muted");
    // once r is used below, the above line to mutate s is invalid since an immutable ref is in scope
    // println!("contract violated for r: {}", r);
    // println!("s: {}", s);
}

// Returns ending index of first word in string
// NOTE: a string slice is of type &str
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    // we can destructure pointers
    for (i, &item) in bytes.iter().enumerate() {
        // we can prefix with b to refer to bytes
        // this is similar to how we refer to numbers in different bases
        // eg: 0x for hex, 0o for oct etc
        if item == b' ' {
            return &s[..i];
        }
    }

    s
}
