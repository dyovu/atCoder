// use proconio::input;
use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    let input = std::io::stdin();

    let mut lines = input.lock().lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for i in 0..t{
        let line  = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let n = line[0];
        let m = line[1];

        let mut sec_a  = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse::<usize>().unwrap()%m).collect::<Vec<_>>();
        let mut sec_b  = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse::<usize>().unwrap()%m).collect::<Vec<_>>();


        /*
         * Aを降順にソートしているので、後ろに行くほどAの値は小さくなります。
         * もしあるA[i]に対してB[j] < M - A[i]（つまりA[i] + B[j] < M）だったら、それ以降のより小さいA[i+1], A[i+2], ...に対してもB[j] + A[i+1] < Mが必ず成り立ちます。
         * だから、一度Mを超えるのに使えなかったBの要素は、以降のAに対しても絶対に使えません。そのためb_startでスキップして、次回の探索では最初から除外できます。
         */
        
        sec_a.sort_by(|a, b| b.cmp(a));
        sec_b.sort();

        let mut count:usize = 0;
        let mut b_start:usize = 0;

        for i in 0..n{
            let k = sec_a.get(i).unwrap();
            let threshold = m - k;
            let pos = b_start + sec_b[b_start..].partition_point(|&x| x < threshold);

            if pos < n {
                count += 1;
                b_start = pos + 1; // 次回はこの位置の次から探す
            }
        }

        println!("{}", sec_a.iter().sum::<usize>() + sec_b.iter().sum::<usize>() -  m*count);


    }



}

