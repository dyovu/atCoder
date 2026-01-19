use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        x: usize,
        y: usize,
    }

    let ans = match (x + y)%12 {
        0 => 12,
        _ => (x + y) % 12,
    };

    println!("{}", ans);



}