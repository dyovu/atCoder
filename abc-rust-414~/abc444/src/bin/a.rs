use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: String
    }

    let mut pre = n.chars().nth(0).unwrap();
    for i in n.chars(){
        if pre != i{
            print!("No");
            return
        }
        pre = i
    }

    print!( "Yes")
}