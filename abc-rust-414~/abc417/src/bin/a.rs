use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        a: usize,
        b: usize,
        s: String,
    }


    println!("{}", s.get(a..n-b).unwrap())

}