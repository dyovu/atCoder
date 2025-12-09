// use proconio::input;
use std::io::{self, BufRead};
use std::collections::BTreeSet;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let t: usize = first_line.parse().unwrap();

    let ans: Vec<isize> = vec![0; t];

    for i in 0..t{
        let num_of_domino = lines.next().unwrap().unwrap();
        let num_of_domino: usize = num_of_domino.parse().unwrap();

        let dominos = lines.next().unwrap().unwrap();
        let dominos_vec: Vec<&str> = dominos.split_whitespace().collect();

        let first:usize = dominos_vec[0].parse().unwrap();
        let last:usize = dominos_vec[num_of_domino-1].parse().unwrap();

        // println!("{}, {}", first, last);
        // print_type(first);
        // print_type(mids);

        // usizeのソート済みのsetに変換する
        // vecに変換してソートしてもいい。 もしかしたらそっちの方がいいのかもしれない
        let mids_set: BTreeSet<usize> = dominos_vec
            .iter()
            .map(|&s| s.parse::<usize>().unwrap())
            .collect();

        // println!("{:?}", mids_set);
        // print_type(mids_set);

        let mut current_domino = first;
        // print_type(current_domino);
        let mut count: isize = 2;
        while current_domino*2 < last{
            let next_domino = *mids_set.range(..=2*current_domino).next_back().unwrap();
            // print_type(next_domino);
            // println!("{}", next_domino);
            count += 1;
            if next_domino == current_domino{
                count = -1;
                break;
            }
            current_domino = next_domino
        }

        println!("{}", count);
        // println!("t={} : {:?}", i, dominos_vec)
    }

}