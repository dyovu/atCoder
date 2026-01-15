use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        a: String,
    }
    if a.len() < 3{
        print!("No");
    }else if a.get(n-3..n).unwrap() == "tea"{
        print!("Yes");
    }else{
        print!("No");
    }

}