use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        d: isize,
        f: isize,
    }
    // println!("{}", (d-f)/7);
    
    println!("{}", f+ 7*((d-f)/7 +1) - d);

}