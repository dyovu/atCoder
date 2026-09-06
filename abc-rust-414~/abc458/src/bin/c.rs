use std::cmp::min;

use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        s: String,
    }

    let mut iter = s.chars();
    let len = s.len();
    let mut ans = 0;
    for (idx, c) in iter.enumerate(){
        if c == 'C'{
            // println!("here");
            ans += (len - idx).min(idx + 1);
        }
    }
    println!("{}", ans);
}
