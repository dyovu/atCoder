// use proconio::input;
use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let _ = lines.next().unwrap().unwrap();
    let set:HashSet<String> = lines.next().unwrap().unwrap().split_whitespace().map(str::to_string).collect();
    let target = lines.next().unwrap().unwrap();

    if set.contains(&target){
        println!("Yes")
    }else{
        println!("No")
    }
}