use proconio::input;
// use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

const four_neighborhood:[[isize; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];


fn main(){
    input!{
        h: usize,
        w: usize,
        maze: [String; h],
    }

    let maze: Vec::<Vec<char>> = maze.iter().map(|x| x.chars().collect()).collect();

    let mut teleporter:HashMap<char, Vec<Vec<usize>>> = HashMap::new();

    for (x, row) in maze.iter().enumerate(){
        for (y, c) in row.iter().enumerate(){
            if !(*c == '.' || *c == '#' ){
                let vec = teleporter.entry(*c).or_insert(Vec::new());
                vec.push(vec![x, y]);
            }
        }
    }

    // println!("{:?}", teleporter);

    let mut queue:VecDeque<Vec<usize>> = VecDeque::new();
    let mut visited:Vec<Vec<bool>> = vec![vec![false;w]; h];
    queue.push_back(vec![0,0,0]);
    visited[0][0] = true;
    let mut ans = -1;


    while let Some(condition) = queue.pop_front(){
        let current_x = condition[0];
        let current_y = condition[1];

        if current_x == h-1 && current_y == w-1{
            ans = condition[2] as isize;
            break
        }

        let c = maze[current_x][current_y];
        // println!("current_x: {}, current_y : {}",current_x, current_y);

        for neighbor in four_neighborhood{
            let next_x = (current_x as isize + neighbor[0]).max(0).min((h-1) as isize) as usize;
            let next_y = (current_y as isize + neighbor[1]).max(0).min((w-1) as isize) as usize;

            // println!("{}, {}", next_x, next_y);

            if !visited[next_x][next_y] && maze[next_x][next_y] != '#'{
                queue.push_back(vec![next_x, next_y, condition[2]+1]);
                visited[next_x][next_y] = true
            }
        }

        if let Some(tp) = teleporter.remove(&c){
            let mut to_tp = Vec::new();
            for i in tp{
                if i != vec![current_x, current_y]{
                    to_tp.push(i);
                }
            }

            for i in to_tp{
                let next_x = i[0];
                let next_y = i[1];

                if !visited[next_x][next_y] && maze[next_x][next_y] != '#'{
                    visited[next_x][next_y] = true;
                    queue.push_back(vec![next_x, next_y, condition[2] + 1]);
                }
            }
        }
    }

    println!("{}", ans);
    
}