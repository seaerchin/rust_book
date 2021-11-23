fn main() {
    let mut x = 5;
    println!("x: {}", x);
    x = 7;
    println!("x: {}", x);

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
}

fn test(x: i32) -> i32 {
    x
}
