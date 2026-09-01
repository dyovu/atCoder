use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        m: usize,
    }

    let mut cnt = 0;
    for i in n..m{
        cnt += 1;
    }
    print!("{}", cnt + 1);

}
