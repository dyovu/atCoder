use proconio::input;
use std::collections::*;

fn main(){
    input!{
        n : i32,
        t : String,
        a : String,
    }

    for i in 0..n{
        if t.chars().nth(i as usize) == Some('o') && a.chars().nth(i as usize) == Some('o'){
            print!("Yes");
            std::process::exit(0)
        }
    }
    print!("No");
}