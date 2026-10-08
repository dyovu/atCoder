use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        s: String,
        t: String,
    }

    let mut itr = t.chars();
    for i in s.chars(){
        let s = itr.next().unwrap();
        if s != i && s != '*'{
            println!("No");
            std::process::exit(0);
        }
    }

    println!("Yes");
}
