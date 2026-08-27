use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        mut x: usize,
        a: [usize; n],
    }

    for i in a.iter(){
        if (i < &x){
            x = *i;
            println!("1");
        }else {
            println!("0");
        }
    }
}
