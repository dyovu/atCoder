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

    for i in n.chars(){
        if i == 'A'{
            print!("A");
        }else {
            print!(".");
        }
    }
}
