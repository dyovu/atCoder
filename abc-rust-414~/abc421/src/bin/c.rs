use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        S: String,
    }

    let mut a_index:Vec<usize> = Vec::new();
    let mut b_index:Vec<usize> = Vec::new();

    for (index, value) in S.chars().enumerate(){
        if value == 'A'{
            a_index.push(index);
        }else {
            b_index.push(index);
        }
    }

    // println!("A {:?}, B {:?}", a_index, b_index);

    let mut even:usize = 0;
    for (i , a_index) in a_index.iter().enumerate(){
        even += ((2*i - a_index) as isize).abs() as usize;
    }

    let mut odd:usize = 0;
    for (i , a_index) in a_index.iter().enumerate(){
        odd += ((2*i+1 - a_index) as isize).abs() as usize;
    }

    // println!("even {}, odd {}", even, odd);

    println!("{}", even.min(odd));

}