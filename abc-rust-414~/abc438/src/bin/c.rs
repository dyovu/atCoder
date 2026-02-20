use proconio::input;
// use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        mut a: [usize; n],
    }

    let mut deque = VecDeque::new();
    let mut remove_cnt:usize = 0;
    for &i in a.iter(){
        deque.push_back(i);
        
        let last_four:Vec<&usize> = deque.iter().rev().take(4).collect();
        if 4 <= deque.len(){
            if last_four[0] == last_four[1] && last_four[1]==last_four[2] && last_four[2]==last_four[3]{
                deque.truncate(deque.len().saturating_sub(4));
                remove_cnt += 4;
            }
        }
        
    }

    println!("{}", n-remove_cnt);

}