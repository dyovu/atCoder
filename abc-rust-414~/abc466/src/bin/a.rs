use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        a: [isize; n],
    }

    for i in a{
        if 0 <= i{
            println!("No");
            std::process::exit(0);
        }
    }
    println!("Yes");
}
