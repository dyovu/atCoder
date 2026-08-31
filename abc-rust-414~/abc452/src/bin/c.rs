use proconio::input;
// use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        a: [[usize; 2]; n],
        m: usize,
        strs: [String; m],
    }

    let mut vec: Vec<HashSet<char>> = vec![HashSet::new(); n];

    for (idx, entry) in a.iter().enumerate(){
        for s in strs.iter(){
            if s.len() == entry[0] {
                vec[idx].insert(s.chars().nth(entry[1] - 1).unwrap());
            }
        }
    }

    // println!("{:?}", vec);
    for i in strs.iter(){
        if n < i.len(){
            println!("No");
            continue;
        }
        let itr = i.chars();
        let mut flag: bool = true;
        for (idx, c) in itr.enumerate(){
            if !vec[idx].contains(&c){
                flag = false;
                break;
            }
        }
        if flag {
            println!("Yes")
        }else {
            println!("No")
        }
    }

}
