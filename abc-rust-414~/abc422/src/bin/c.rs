use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        t: usize,
        a: [[usize; 3]; t],
    }

    // println!("{:?}", a);

    for case in a.iter(){
        let an = case[0];
        let bn = case[1];
        let cn = case[2];
        let min_a_c = an.min(cn);

        if min_a_c <= bn + ((an - cn) as isize).abs() as usize{
            println!("{}", min_a_c);
            continue
        }

        let rem = min_a_c - bn - ((an - cn) as isize).abs() as usize;
        let a_c = (rem*2)/3;
        // println!("rem {}, a_c {}", rem, a_c);
        println!("{}", a_c + min_a_c - rem);
    }
    

}