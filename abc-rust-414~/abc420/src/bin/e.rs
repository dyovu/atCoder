// use proconio::input;
use std::{default, io::{self, BufRead}};
// use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    black_count: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
            black_count: vec![0; n],
        }
    }
    
    // 経路圧縮による親の検索
    fn find(&mut self, x: usize) -> usize {
	    if self.parent[x] != x {
	        self.parent[x] = self.find(self.parent[x]);
	    }
	    self.parent[x]
	}
	
	// rankを用いた結合
	fn union(&mut self, x: usize, y: usize) {
	    let root_x = self.find(x);
	    let root_y = self.find(y);
	    
	    if root_x == root_y {
	        return;
	    }

        let count_sum = self.black_count[root_x] + self.black_count[root_y];
	    
	    if self.rank[root_x] < self.rank[root_y] {
	        self.parent[root_x] = root_y;
            self.black_count[root_y] = count_sum;
            self.black_count[root_x] = 0;
	    } else if self.rank[root_x] > self.rank[root_y] {
	        self.parent[root_y] = root_x;
            self.black_count[root_x] = count_sum;
            self.black_count[root_y] = 0;
	    } else {
	        self.parent[root_y] = root_x;
	        self.rank[root_x] += 1;
            self.black_count[root_x] = count_sum;
            self.black_count[root_y] = 0;
	    }
	}

    // 頂点の色を反転させる
    fn revers_color(&mut self, x: usize, is_black: &mut Vec<bool>){
        let root = self.find(x);
        
        if is_black[x] {
            self.black_count[root] -= 1;
            is_black[x] = false;
        } else {
            self.black_count[root] += 1;
            is_black[x] = true;
        }
    }

    // その頂点を含むグループに黒い頂点があるか
    fn has_black (&mut self, x: usize,) -> bool{
        let root = self.find(x);
        self.black_count[root] > 0
    }
}

fn main(){
    let stdin = std::io::stdin();

    let mut lines = stdin.lock().lines();
    let first = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let n = first[0];
    let q = first[1];

    let mut is_black = vec![false; n+1];
    let mut union_find = UnionFind::new(n+1);


    for i in lines.take(q){
        // println!("{:?}",i);
        let line = i.unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let query_type = line[0];
        
        match query_type {
            1 => {
                let u = line[1];
                let v = line[2];
                union_find.union(u, v);
            }
            2 => {
                let u = line[1];
                union_find.revers_color(u, &mut is_black);
            }
            3 => {
                let u = line[1];
                match union_find.has_black(u){
                    true => {println!("Yes")}
                    false => {println!("No")}
                }
            }
            _ => {}
        }
        
    }

}