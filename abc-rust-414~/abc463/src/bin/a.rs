use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        m: usize,
    }
    
    if n * 9 == m * 16{
        println!("Yes");
    }else {
        print!("No");
    }
}
