use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: String,
    }

    let len = n.len();

    println!("{}{}", &n[0..(len+1)/2-1], &n[(len+1)/2..])

}