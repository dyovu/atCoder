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
        if '0' as u32 <= i as u32 && i as u32 <= '9' as u32{
            print!("{}", i);
        }
    }
}
