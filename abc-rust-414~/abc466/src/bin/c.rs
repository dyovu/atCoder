use std::io::BufRead;

use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let mut i = 1;
    let mut j = 2;
    let mut cnt: usize = 0;

    while i < n{
        println!("? {} {}", i, j);
        let resp = lines.next().unwrap().unwrap();
        match resp.as_str(){
            "Yes" => {
                if j == n{
                    cnt += 1;
                    i += 1;
                }else{
                    cnt += j - i;
                    j += 1;
                }
                // println!("cnt: {}", cnt);
            }
            "No" => {
                if j - i == 1{
                    j += 1;
                }
                i += 1; 
            }
            _ => {}
        }
    }
    println!("! {}", cnt);
}
