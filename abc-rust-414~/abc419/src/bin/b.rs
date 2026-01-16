// use proconio::input;
use std::io::{self, BufRead};
use std::collections::*;
use std::cmp::Reverse;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    let input = std::io::stdin();
    let mut lines = input.lock().lines();
    let q: usize = lines.next().unwrap().unwrap().parse().unwrap();

    let mut heap = BinaryHeap::new();


    for i in 0..q{
        let line = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();

        match line[0]{
            1  => {heap.push(Reverse(line[1]))}
            2  => {
                if let Some(Reverse(val)) = heap.pop() {
                    println!("{}", val);
                }
            }
            _  => {}
        }
    }

    // println!("{:?}", heap)
}