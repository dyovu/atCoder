
use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize; 2]; m],
        q: usize,
        query: [[usize; 2]; q],
    }

    // 逆グラフを構築
    let mut rev_graph: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    for i in &a {
        let start = i[0];
        let end = i[1];
        rev_graph[end].push(start); // 辺を逆向きに張る
    }

    let mut reachable: Vec<bool> = vec![false; n + 1];

    // 黒頂点にしたとき、逆グラフ上でBFSしてreachableを伝播
    let mut propagate = |v: usize, rev_graph: &Vec<Vec<usize>>, reachable: &mut Vec<bool>| {
        if reachable[v] {
            return; // 既にreachableなら何もしない
        }
        reachable[v] = true;
        let mut queue = VecDeque::new();
        queue.push_back(v);
        while let Some(cur) = queue.pop_front() {
            for &prev in &rev_graph[cur] {
                if !reachable[prev] {
                    reachable[prev] = true;
                    queue.push_back(prev);
                }
            }
        }
    };

    for i in &query {
        let query_type = i[0];
        let v = i[1];
        match query_type {
            1 => {
                propagate(v, &rev_graph, &mut reachable);
            }
            2 => {
                if reachable[v] {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
            _ => {}
        }
    }
}




// use proconio::input;
// // use std::io::{self, BufRead};
// use std::collections::*;

// fn print_type<T>(_: T) {
//     println!("{}", std::any::type_name::<T>());
// }


// fn is_reach_black(
//     v: usize,
//     graph: &Vec<Vec<usize>>,
//     reachable: &mut Vec<bool>,
//     checked_gen: &mut Vec<u32>,
//     current_gen: u32,
// ) -> bool {
//     if checked_gen[v] == current_gen {
//         return false; // この世代で既に到達不可と確認済み
//     }

//     let mut visited = Vec::new();
//     let mut stack = VecDeque::new();
//     let mut parent: HashMap<usize, usize> = HashMap::new();

//     stack.push_front(v);
//     visited.push(v);

//     let mut visited_set = HashSet::new();
//     visited_set.insert(v);

//     while let Some(vertex) = stack.pop_front() {
//         if reachable[vertex] {
//             let mut cur = vertex;
//             while cur != v {
//                 reachable[cur] = true;
//                 cur = parent[&cur];
//             }
//             reachable[v] = true;
//             return true;
//         }
//         for &next in graph[vertex].iter() {
//             // checked_genが現世代の頂点もスキップ
//             if !visited_set.contains(&next) && checked_gen[next] != current_gen {
//                 visited_set.insert(next);
//                 visited.push(next);
//                 parent.insert(next, vertex);
//                 stack.push_front(next);
//             }
//         }
//     }

//     // 到達不可だった頂点全てに現世代を記録
//     for &u in &visited {
//         checked_gen[u] = current_gen;
//     }
//     false
// }


// fn main(){
//     input!{
//         n: usize,
//         m: usize,
//         a: [[usize; 2]; m],
//         q: usize,
//         query: [[usize; 2]; q],
//     }

//     let mut graph:Vec<Vec<usize>> = vec![Vec::new(); n+1];
//     let mut reachable:Vec<bool> = vec![false; n+1];

//     for i in a{
//         let start = i[0];
//         let end = i[1];
//         // println!("start: {}, end: {}", start, end);
//         graph[start].push(end);
//     }

//     // println!("{:?}", graph);

//     let mut checked_gen: Vec<u32> = vec![0; n + 1];
//     let mut current_gen: u32 = 1; 

//     for i in query{
//         let qyery_type = i[0];
//         let v = i[1];
//         match qyery_type{
//             1 => {
//                 reachable[v] = true;
//                 current_gen += 1;
//             }
//             2 => {
//                 if is_reach_black(v, &graph, &mut reachable, &mut checked_gen, current_gen){
//                     println!("Yes")
//                 }else{
//                     println!("No")   
//                 }
//             }
//             _ => {}
//         }
//     }
// }

