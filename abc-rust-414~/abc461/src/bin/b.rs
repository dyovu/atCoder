use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }
    
    for (ki, &ax) in a.iter().enumerate(){
        if b[ax - 1] != ki + 1{
            println!("No");
            std::process::exit(0);
        }
    }
    println!("Yes");
}
