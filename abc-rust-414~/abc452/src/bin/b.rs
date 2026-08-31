use core::hash;

use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        h: usize,
        w: usize,
    }

    for i in 0..h{
        for j in 0..w{
            if i == 0 || i == h - 1 || j == 0 || j == w - 1{
                print!("#");
            }else {
                print!(".");
            }
        }
        println!();
    }

}
