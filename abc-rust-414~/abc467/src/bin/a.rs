use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        w: usize,
    }

    if n * n * 25 <= w * 100 * 100 {
        println!("Yes")
    }else {
        println!("No")
    }
}
