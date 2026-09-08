use proconio::input;
// use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        mut k: usize,
        m: usize,
        mut map: [[usize; 2]; n],
    }

    map.sort_by(|a, b| b[1].cmp(&a[1]));
    let mut ans = 0;
    let mut free = k - m;
    let  mut picked: HashSet<usize> = HashSet::new();

    for i in map{
        if !picked.contains(&i[0]){
            ans += i[1];
            picked.insert(i[0]);
            k -= 1;
        }else {
            if 0 < free{
                ans += i[1];
                free -= 1;
                k -= 1;
            }
        }
        if k == 0{
            break;
        }
    }
    println!("{}", ans);
}
