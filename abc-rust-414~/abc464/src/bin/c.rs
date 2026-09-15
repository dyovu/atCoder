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
        mut repo: [[usize; 3]; n],
    }
    
    repo.sort_by(|a, b| a[1].cmp(&b[1]));
    // println!("{:?}", repo);
    

    let mut map: HashMap<usize, usize> = HashMap::new(); 
    for i in repo.iter(){
        *map.entry(i[0]).or_insert(0) += 1;
    }

    let mut idx = 0;
    for i in 0..m{
        if idx == n{
            println!("{}", map.len());
            continue
        }
        while repo[idx][1] == i + 1{
            let entry = map.entry(repo[idx][0]).or_insert(1);
            *entry -= 1;
            if *entry == 0{
                map.remove(&repo[idx][0]);
            }
            *map.entry(repo[idx][2]).or_insert(0) += 1;
            idx += 1;
        }
        println!("{}", map.len());
    }
}
