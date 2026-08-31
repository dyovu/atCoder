use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        str: String,
    }
    
    let mut iter = str.chars();

    for i in iter.by_ref(){
       if i != 'o'{
            print!("{}", i);
            break;
        }
    }

    for i in iter{
        print!("{}", i);
    }
    println!();
}
