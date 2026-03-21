// use proconio::input;
use std::io::{self, BufRead};
use std::collections::*;
use std::cmp::Reverse;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    let mut lines = std::io::stdin().lock().lines();
    let t = lines.next().unwrap().unwrap().parse::<usize>().unwrap();

    for _ in 0..t{
        let n = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
        let row:Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();

        let mut priority_queue:BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new();

        for (i, v) in row.iter().enumerate(){
            priority_queue.push(Reverse((*v, i)));
        }

        let mut a = row.clone();
        while !priority_queue.is_empty(){
            let Reverse((ai, i)) = priority_queue.pop().unwrap();
            if ai != a[i] { continue }

            if i != 0 && a[i - 1] > a[i] + 1 {
				a[i - 1] = a[i] + 1;
				priority_queue.push(Reverse((a[i - 1], i - 1)));
			}
			if i != n - 1 && a[i + 1] > a[i] + 1 {
				a[i + 1] = a[i] + 1;
				priority_queue.push(Reverse((a[i + 1], i + 1)));
			}
        }

        let mut diff = 0;
        for i in 0..n{
            diff += row[i] - a[i];
        }
        println!("{}", diff);

    }

}