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
        mut a: [usize; n],
        b: [usize; q],
    }
    a.sort();
    // println!("{:?}", a);
    
    // 先に特定のindexまでの累積和を求めておく
    let mut prefix_sum: Vec<usize> = vec![0; n];
    let mut current_sum :usize= 0;
    for (index, value) in a.iter().enumerate(){
        prefix_sum[index] = current_sum + value;
        current_sum += value;
    }
    // println!("{:?}", prefix_sum);

    for i in b{
        let over_i_index = a.partition_point(|&x| x < i);
        if a[n-1] < i {
            println!("{}", -1);
            continue
        }
        // println!("under_i_index : {}, b {}", over_i_index, i);

        let mut ans:usize = if over_i_index == 0 {0}else{prefix_sum[over_i_index-1]};
        ans += (n-over_i_index)*(i-1) + 1;
        println!("{}", ans);
    }
}