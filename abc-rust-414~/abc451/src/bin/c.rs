use proconio::input;
// use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main(){
    input!{
        n: usize,
        query: [[usize; 2]; n],
    }

    let mut heap = BinaryHeap::new();

    for q in query{
        match q[0]{
            1 => {
                heap.push(Reverse(q[1]));
                println!("{}", heap.len());
            }
            2 => {
                while heap.peek() != None{
                    if  heap.peek().unwrap().0 <= q[1] {
                        heap.pop();
                    }else {
                        break;
                    }
                }
                println!("{}", heap.len());
            }
            _ => {}
        } 
    }
}
