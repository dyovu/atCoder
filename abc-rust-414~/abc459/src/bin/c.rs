use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        q: usize,
        query: [[usize; 2]; q],
    }
    let mut idx_map: Vec<usize> = Vec::from_iter(0..n);
    let mut idx_map_rev: Vec<usize> = Vec::from_iter(0..n);
    let mut seq: Vec<usize> = vec![0; n];
    let mut virt_del = 0;

    for i in query{
        // println!("idx: {}, seq idx: {}", i[1] - 1, idx_map[i[1] - 1]);
        match i[0]{
            1 => {
                let idx = idx_map[i[1] - 1];
                let cnt = seq[idx];
                let new_idx = seq.partition_point(|&x| x < cnt + 1) - 1;
                // println!("new idx: {}", new_idx);
                if new_idx == 0{
                    virt_del += 1;
                }
                seq[new_idx] += 1;
                let tmp = idx_map[i[1] - 1];
                idx_map[i[1] - 1] = new_idx;
                idx_map[idx_map_rev[new_idx]] = tmp;

                let tmp_rev = idx_map_rev[new_idx];
                idx_map_rev[new_idx] = i[1] - 1;
                idx_map_rev[tmp] = tmp_rev;
            }
            2 => {
                let cnt = i[1];
                let idx = seq.partition_point(|&x| x - virt_del < cnt);
                println!("{}", n - idx);
            }
            _ => {}
        }
        // println!("seq: {:?}", seq);
        // println!("idx map: {:?}, idx_map_rev:{:?}", idx_map, idx_map_rev);

        // println!();
    }
}
