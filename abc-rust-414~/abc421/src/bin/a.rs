use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        s: [String; n],
        x: usize,
        y: String,
    }

    // println!("{:?}", s[x-1]);

    if s[x-1] == y{
        println!("Yes");
    }else{
        println!("No");
    }

}