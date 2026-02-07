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
        a: [usize; n],
    }

    let sum:usize = a.iter().sum();
    let dif = sum as isize - m as isize;

    if dif < 0{
        println!("No")
    }else{
        if a.contains(&(dif as usize)){
            println!("Yes")
        }else{
            println!("No")
        }
    }



}