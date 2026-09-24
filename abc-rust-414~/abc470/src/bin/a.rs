use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
    }

    for i in 1..=n{
        if i % 3 == 0{
            println!("Fizz");
        }else {
            println!("{}", i);
        }
    }
}
