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
    println!("{}",x*2usize.pow(y as u32));



}