use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        a: [usize; n],
    }

    let mut max_pos:usize = 1 + a[0];

    for (index, value) in a.iter().enumerate(){
        // println!("max_pos: {}", max_pos);
        let pos = index+1;

        if max_pos <= pos{
            break
        }
        

        max_pos = max_pos.max(*value+ pos);

        if n < max_pos{
            max_pos = n+1;
            break
        }
    }

    println!("{}", max_pos-1);

}