use std::io::BufRead;

use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    let mut lines = std::io::stdin().lock().lines();
    let t = lines.next().unwrap().unwrap().parse::<usize>().unwrap();

    for i in 0..t{
        let situation:Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
        let n = situation[0];
        let w = situation[1];
        let c:Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();

        if n <= w {
            println!("{}", 0);
            continue
        }

        let ww = 2*w;

        let mut cost_per_rem:Vec<usize> = vec![0; ww];
        for (index, cost) in c.iter().enumerate(){
            cost_per_rem[index%ww] += cost;
        }

        // println!("{:?}", cost_per_rem);

        let partial_sum_num = w;


        // println!("partial_sum_num: {:?}", partial_sum_num);

        let mut partial_sum:usize = cost_per_rem[0..partial_sum_num].iter().sum();
        // println!("partial_sum: {}", partial_sum);
        let mut partial_sum_min:usize = partial_sum;

        for index in 0..ww{
            partial_sum = partial_sum - cost_per_rem[index] + cost_per_rem[(index+partial_sum_num)%ww];
            if partial_sum < partial_sum_min{
                partial_sum_min = partial_sum;
            }
        }

        println!("{}", partial_sum_min);


    }


}