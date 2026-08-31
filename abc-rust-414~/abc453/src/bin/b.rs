use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        t: usize,
        x: usize,
        a: [usize; t + 1],
    }
    
    let mut last_recorded = 0;
    for (time, &value) in a.iter().enumerate(){
       if time == 0{
            println!("{} {}", 0, value);
            last_recorded = value;
        }else if x <= last_recorded.abs_diff(value) {
            println!("{} {}", time, value);
            last_recorded = value;
        }
        println!("last_recorded: {}, value: {}", last_recorded, value);
    }

}
