use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        k: usize,
    }

    if n <= k{
        println!("0");
    }else{
        println!("{}", n - k);
    }
}