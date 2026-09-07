use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        t: usize,
        a: [[usize; 6]; t],
    }

    for i in a{
        let d = (i[3] - i[0]).pow(2) + (i[4] - i[1]).pow(2);
        if d <= (i[2] + i[5]).pow(2) && (i[2] - i[5]).pow(2) <= d{
            println!("Yes");

        }else {
            println!("No");
        }
    }

}
