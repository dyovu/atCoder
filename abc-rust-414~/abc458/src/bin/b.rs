use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        h: usize,
        w: usize,
    }

    for i in 0..h{
        let mut x: usize = 0;
        if i == 0 { x += 1; } 
        if i == h - 1 { x += 1;}
            
        for j in 0..w{
            if j != 0{
                print!(" ");
            }
            let mut y = 0;
            if j == 0 { y += 1; } 
            if j == w - 1 { y += 1;}

            print!("{}", 4 - x - y);
        }
        println!();
    }

}
