use std::io;

fn main() {
    let mut s = "All he could think about was how it would all end. There was
    still a bit of uncertainty in the equation, but the basics
    were there for anyone to see. No matter how much he tried to
    see the positive, it wasn't anywhere to be seen. The end was
    coming and it wasn't going to be pretty.".to_string();

    let l: Vec<String> = Vec::new();
    let mut stop_index = 0;

    s.trim();
    for i in 0..s.len()-1 {
        if s[i] == " " {
            let w = String::new();
            for i in stop_index..i {
                let x = i as i32;
                w.push(s[x]);
            }
        }
    }


}
