use proconio::input;
use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let mut map: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (idx, part) in lines.enumerate(){
        let line: Vec<usize> = part.unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
        println!("{:?}", line);
        for j in 1..line.len(){
            map[line[j] - 1].push(idx + 1);
        }
    }

    for i in map{
        print!("{}", i.len());
        for j in i{
            print!(" {}", j);
        }
        println!();
    }
}
