use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        p: usize,
        q: usize,
        x: usize,
        y: usize,
    }

    if p <= x && x < p+100 && q <= y && y < q+100 {
        println!("Yes")
    } else {
        println!("No")
    }
}