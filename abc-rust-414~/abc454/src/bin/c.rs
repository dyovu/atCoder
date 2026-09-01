use std::{collections::{HashSet, VecDeque}, hash::Hash};

use proconio::input;
// use std::io::{self, BufRead};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        m: usize,
        a: [[usize; 2]; m],
    }

    let mut graph = vec![Vec::new(); n + 1];

    for i in a{
        graph[i[0]].push(i[1]);
    }
    // println!("{:?}", graph);

    let mut cnt = 0;
    let mut visited = vec![0; n + 1];
    let mut node: VecDeque<usize> = VecDeque::new();
    node.push_back(1);

    while node.len() != 0{
        let i = node.pop_front().unwrap();
        if visited[i] == 1{
            continue
        }
        visited[i] = 1;
        cnt += 1;
        for j in &graph[i]{
            if visited[*j] != 1{
                node.push_back(*j);
            }
        }
    }

    println!("{}", cnt);

}
