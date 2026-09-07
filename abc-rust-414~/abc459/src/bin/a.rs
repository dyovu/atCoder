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

    let str = "HelloWorld";
    for (idx ,c) in str.chars().enumerate(){
        if idx == n - 1{
            continue ;
        }
        print!("{}", c);
    }

}
