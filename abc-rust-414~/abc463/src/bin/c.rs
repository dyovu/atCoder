use proconio::input;
// use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        mut map: [[usize; 2]; n],
        q: usize,
        mut time: [usize; q],
    }

    map.sort_by(|a, b| b[0].cmp(&a[0]));
    let mut ans: HashMap<usize, usize> = HashMap::new();
    let mut time_sorted = time.clone();
    time_sorted.sort();
    println!("map{:?}", map);
    println!("time_sorted{:?}",time_sorted);
    let mut idx = 0;
    for i in time_sorted.iter(){
        while map[idx][1] <= *i{
            idx += 1;
        }
        ans.insert(*i, map[idx][0]);
    }
    println!("ans{:?}",ans);
    for i in time.iter(){
        println!("{}", ans[i]);
    }
}
