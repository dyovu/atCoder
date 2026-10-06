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
    }

    for (idx, &val) in a.iter().enumerate(){
        if (idx / 10 + 1) * 10 < val{
            println!("No");
            std::process::exit(0);
        }
    }
    println!("Yes");
}
