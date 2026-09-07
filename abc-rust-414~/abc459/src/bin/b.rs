use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        strs: [String; n],
    }

    for i in strs{
        let num = i.chars().next().unwrap() as u32;
        if num < 's' as u32{
            print!("{}", (num - 91) / 3);
        }else if num == 's' as u32{
            print!("7");
        }else if num <= 'v' as u32{
            print!("8");
        }else{
            print!("9");
        }
    }
}
