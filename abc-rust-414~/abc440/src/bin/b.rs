use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        mut t: [usize; n],
    }

    // t.sort();
    let mut map:Vec<(usize, usize)> = t.iter().enumerate().map(|(index, &time)| (time, index)).collect();
    map.sort_by_key(|&(k,_)| k);

    // println!("{:?}", map);

    for i in 0..3{
        print!("{} ", map[i].1+1);
    }

}