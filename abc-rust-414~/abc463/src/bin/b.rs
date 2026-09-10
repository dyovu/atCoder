use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        x: char,
        seats: [String; n],
    }
    
    let no = x as usize - 65;
    for i in seats{
        if i.chars().nth(no).unwrap() == 'o'{
            println!("Yes");
            std::process::exit(0);
        }
    }
    println!("No");
}
