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

    let first_char = s.chars().nth(0).unwrap();
    let mut other_char:char = ' ';
    let mut count = 0;

    for i in s.chars(){
        if i == first_char{
            count += 1;
        }else{
            other_char = i;
        }
    }

    if 1 < count{
        println!("{}", other_char);
    }else{
        println!("{}", first_char);
    }

}