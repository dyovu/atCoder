use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        q: usize,
        mut a: [usize; n],
        mut b: [usize; n],
        query: [(char, usize, usize); q],
    }
    let mut min_sum = 0;
    for i in 0..n{
        min_sum += a[i].min(b[i]);
    }

    // println!("{}", min_sum);

    for (c, x, y) in query.iter(){
        let index = x-1;
        min_sum -= a[index].min(b[index]);
        match c{
            'A' => {
                a[index] = *y;
            }
            'B' => {
                b[index] = *y;
            }
            _ => {}
        }
        min_sum += a[index].min(b[index]);
        println!("{}", min_sum);
    }
    // println!("{}", min_sum);
}