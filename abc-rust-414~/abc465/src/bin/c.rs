use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        s: String,
    }

    let mut vec: Vec<bool> = vec![false; n];
    let seq: Vec<usize> = (1..=n).collect();
    for (idx, c) in s.chars().enumerate(){
        if c == 'o' {
            vec[idx] = true;
        }
    }

    let mut is_rev: bool = false;
    let mut ans1 = Vec::new();
    let mut ans2 = Vec::new();
    for (idx, &val) in vec.iter().rev().enumerate(){
        is_rev = (val != is_rev) && (val || is_rev);
        if is_rev{
            ans1.push(seq[n - 1 - idx]);
        }else {
            ans2.push(seq[n - 1  - idx]);
        }
    }
    for (idx, val) in ans1.iter().enumerate(){
        if idx != 0{
            print!(" ");
        }
        print!("{}", val);
    }
    for i in ans2.iter().rev(){
        print!(" {}", i);
    }
}
