use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    if a <= c{
        if d < b{
            println!("Yes");
            return
        }
    }
    println!("No")

}