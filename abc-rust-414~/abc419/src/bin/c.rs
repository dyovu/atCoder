use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        cordinate: [[usize; 2]; n],
    }

    let mut x:Vec<usize> = cordinate.iter().map(|x| x[0]).collect();
    let mut y:Vec<usize> = cordinate.iter().map(|x| x[1]).collect();

    x.sort();
    y.sort();

    // println!("x: {:?}, y: {:?} ", x, y);

    let x_diff = (x[n-1] - x[0]+1)/2;
    let y_diff = (y[n-1] - y[0]+1)/2;

    println!("{}", x_diff.max(y_diff));

}