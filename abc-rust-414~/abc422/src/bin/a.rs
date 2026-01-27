use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        s: String,   
    }

    let i = s.chars().nth(0).unwrap().to_digit(10).unwrap();
    let j = s.chars().nth(2).unwrap().to_digit(10).unwrap();

    if j == 8{
        println!("{}-{}", i+1, 1);
    }else{
        println!("{}-{}", i, j+1);
    }


}