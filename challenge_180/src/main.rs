use std::collections::HashMap;
fn main() {
    let s = String::from("aabvbcc");
    first_unique(&s)
}

fn first_unique(s: &String) {
    println!("{s}");
    let mut occurance_map = HashMap::new();
    for c in s.chars() {
        match occurance_map.insert(c, 1) {
            Some(x) => {occurance_map.insert(c, x + 1);}
            None => {}
        }
    }
    occurance_map.remove(&' ');
    for c in s.chars() {
        match occurance_map.get(&c) {
            Some(x) => {
                if x == &1 {
                    println!("{} is the first unique character!", &c);
                    return;
                }
            }
            None => (),
        }
    }
    println!("there are no unique characters!");
    println!("{:?}", occurance_map);
}
