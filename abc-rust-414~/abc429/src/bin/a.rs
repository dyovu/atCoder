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

    for i in 0..n{
        if i+1 <= m{
            println!("OK")
        }else{
            println!("Too Many Requests")
        }
    }

}