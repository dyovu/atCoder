use proconio::input;
// use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        m: usize,
        s: String,
        t: String,
        q: usize,
        words: [String; q],
    }

    let set_s = s.chars().collect::<HashSet<_>>();
    let set_t = t.chars().collect::<HashSet<_>>();

    for i in words{
        let mut is_s = true;
        let mut is_t = true;

        for j in i.chars(){
            if !set_s.contains(&j) {
                is_s = false;
            }
            if !set_t.contains(&j) {
                is_t = false;
            }
        }

        if is_s && !is_t {
            println!("Takahashi")
        } else if is_t && !is_s {
            println!("Aoki")
        } else {
            println!("Unknown")
        }
    }

}