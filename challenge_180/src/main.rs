use std::collections::HashMap;
fn main() {
    let s = String::from("aabvbcc");
    let  test = vec![1,4,2,3,5];
    first_unique(&s);
    trim_list(&test, 3);
    //println!("{:?}",test);
 
   
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

fn trim_list(l: &Vec<i32>, val: i32) -> Vec<i32> {
    let mut new_vec: Vec<i32> = Vec::new();
    for  i in 0..l.len()-1 {
        if l[i] >= val {
           new_vec.push(l[i])
        }
    }
    println!("{:?}", new_vec);
    return new_vec;
}
