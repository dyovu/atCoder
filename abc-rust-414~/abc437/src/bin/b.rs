use proconio::input;
// use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        h: usize,
        w: usize,
        n: usize,
        a: [[usize; w]; h],
        b: [usize; n],
    }

    let a_set: Vec<HashSet<usize>> = a
        .iter()
        .map(|x| x.iter().map(|x| *x).collect::<HashSet<usize>>())
        .collect();

    let mut max_cnt= 0;
    for i in 0..h{
        let row = &a_set[i];
        let mut cnt = 0;
        for j in b.iter(){
            if row.contains(j){
                cnt += 1;
            }
        }
        max_cnt = max_cnt.max(cnt);
    }

    println!("{}", max_cnt);

}