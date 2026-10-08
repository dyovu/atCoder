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

    for (idx, c) in n.chars().enumerate(){
        print!("{}", c);
        if idx == n.len() - 1{
            if c == 'e'{
                print!("r");
            }else{
                print!("er");
            }
        }

    }
}
