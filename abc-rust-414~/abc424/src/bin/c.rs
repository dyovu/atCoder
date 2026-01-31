use proconio::input;
// use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        a: [[usize; 2]; n],
    }

    let mut visited  = vec![false; n+1];

    let mut graph:Vec<Vec<usize>> = vec![vec![]; n+1];

    for (index, value) in a.iter().enumerate(){
        let k = value[0];
        let l = value[1];

        if k==l {
            graph[k].push(index+1);
            continue;
        }
        graph[k].push(index+1);
        graph[l].push(index+1);
    }

    // println!("{:?}", graph);

    let mut queue:VecDeque<usize> = VecDeque::new();
    queue.push_back(0);

    while 0 < queue.len(){
        let index = queue.pop_front().unwrap();

        // visited[index] = true;
        let tmp = &graph[index];
        for i in tmp{
            if !visited[*i]{
                queue.push_back(*i);
                visited[*i] = true;
            }
        }
    }

    // println!("{:?}",visited);

    let mut count:usize =0;
    for i in visited{
        if i {count += 1}
    }

    println!("{}", count-1)
    

}