use std::collections::HashMap;
fn main() {
    let s = String::from("Hestllo world");
    first_unique(&s)
}

fn first_unique(s: &String) {
    println!("{s}");
    let mut occurance_map = HashMap::new();
    for c in s.chars() {
        match occurance_map.insert(c, 1) {
            Some(x) => {occurance_map.insert(c, x + 1);}
            None         => {}
        }
    }

    for c in s.chars() {
        match occurance_map.get(&c) {
            Some(x) => {
                if x == &1 {println!("{} is the first unique character!", &c);return;}}
            None          => (),
        }
    }
    println!("{:?}", occurance_map);
}
