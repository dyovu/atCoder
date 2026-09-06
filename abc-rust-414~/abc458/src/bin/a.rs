use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        s: String,
        n: usize,
    }

    let len = s.len();
    let mut iter = s.chars();
    for _ in 0..n{
        iter.next();
    }
    for i in 0..len - 2 * n{
        let c = iter.next().unwrap();
        print!("{}", c);
    }
}
