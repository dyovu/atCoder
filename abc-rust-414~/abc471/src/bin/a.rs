use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: f64,
        m: f64,
    }

    if n + m == 9. || n - m == 9. || n * m == 9. || n / m == 9.{
        println!("Nine");
    }else {
        println!("Nein");
    }
}
