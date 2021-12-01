use std::collections::HashMap;

fn main() {
    let mut vec = vec![1, 2, 3, 4];
    let mut value = &vec[1];
    mutate_vec(&mut vec);
    println!("value: {:#?}", vec);
    let mut hm: HashMap<_, _> = vec![(1, 3), (2, 4)].into_iter().collect();
    let mut first = &hm[&1];
    let mut second = hm.get_mut(&2);
    match second {
        None => (),
        Some(v) => {
            *v = 1289347289;
        }
    }
    first = &2341234;
    println!("hashmap: {:#?}", hm);
    println!("first; {}", first);
}

fn mutate_vec(v: &mut Vec<i32>) {
    for item in v {
        *item += 1
    }
}
