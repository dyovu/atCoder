use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: String,
    }
    
    let mut w = 0;
    let mut e = 0;
    for i in n.chars(){
        if i == 'W'{
            w += 1;
        }else{
            e += 1;
        }
    }
    if w < e{
        println!("East");
    }else {
        println!("West");
    }
}
