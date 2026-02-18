use proconio::input;
// use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        m: usize,
        q: [[usize; 2]; m],
    }

    let mut blocked_pos: HashSet<Vec<usize>> = HashSet::new();
    let mut count = 0;

    'outer: for pos in q{
        
        for j in 0..2{
            for k in 0..2{
                if blocked_pos.contains(&vec![pos[0]+j, pos[1]+k]){
                    continue 'outer;
                }
            }
        }

        for j in 0..2{
            for k in 0..2{
                blocked_pos.insert(vec![pos[0]+j, pos[1]+k]);
            }
        }
        count += 1;
    }

    println!("{}", count);

}