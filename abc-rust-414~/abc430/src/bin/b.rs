use proconio::input;
// use std::io::{self, BufRead};
use std::{collections::*, hash::Hash};

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        m: usize,
        a: [String; n],
    }

    let mut set:HashSet<String> = HashSet::new();

    for i in 0..=n-m{
        for j in 0..=n-m{
            let mut s = String::new();

            // println!("{}, {}", i, j);
            for k in 0..m{
                s.push_str(a[i+k].get(j..j+m).unwrap());
                // println!("k={}, s={:?}", k, s);
            }

            // println!("{:?}", s);

            if !set.contains(&s){
                set.insert(s);
            }
        }
    }

    println!("{}", set.len());

}