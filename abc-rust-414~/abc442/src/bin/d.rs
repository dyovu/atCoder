use proconio::input;
use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    let mut lines = std::io::stdin().lock().lines();

    let first_line: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
    let n = first_line[0];
    let q = first_line[1];

    let mut sequence: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
    let mut cum_sum = vec![0; n+1];

    for (i, v) in sequence.iter().enumerate(){
        cum_sum[i+1] = v + cum_sum[i];
    }

    // println!("{:?}", cum_sum);

    for i in lines.take(q){
        let query: Vec<usize> = i.unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
        let query_type = query[0];

        match query_type{
            1 => {
                let x = query[1];
                
                cum_sum[x] += sequence[x] - sequence[x-1];

                let tmp = sequence[x-1];
                sequence[x-1] = sequence[x];
                sequence[x] = tmp;
            }
            2 => {
                let l = query[1];
                let r = query[2];
                let sub_sum = cum_sum[r] - cum_sum[l-1];

                println!("{}", sub_sum);
            }
            _ => {}
        }
    }
    

}