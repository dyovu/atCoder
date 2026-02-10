use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        mut a: [usize; 3],
    }

    a.sort();
    a.reverse();

    println!("{}{}{}", a[0],a[1],a[2]);
    

}