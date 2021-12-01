use std::collections::HashMap;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        // ref converts a value into its reference to avoid borrowing
        // pattern matching using &<value> instead gets the value that the & is referencing
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => {
                panic!("Tried to create file but there was a problem: {:?}", e)
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    };
}

fn get_mut_vec(i: &mut Vec<i32>) -> &mut Vec<i32> {
    i
}

fn get_immutable_vec(i: &mut Vec<i32>) -> &Vec<i32> {
    i
}

fn test() {
    let s: Vec<i32> = Vec::new();
    let mut t: Vec<i32> = vec![1, 2, 3];
    let j = get_mut_vec(&mut t);
    // j.push(3);
    // j = &Vec::new();
    j[0] = 2;
    let k = get_mut_vec(&mut t);
    let mut x = get_immutable_vec(&mut t);
    // x[0] = 1;
    x = &vec![1, 2, 3];
    ()
}
