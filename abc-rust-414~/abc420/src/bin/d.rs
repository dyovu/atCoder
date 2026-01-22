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
        a: [String; h],
    }

    // println!("{:?}", a);

    let mut grid: Vec<Vec<char>> = vec![vec![' '; w]; h];
    let mut start_condition:Vec<isize> = Default::default();
    for (i, value) in a.iter().enumerate(){
        for (j, c) in value.chars().enumerate(){
            grid[i][j] = c;
            if c == 'S'{
                start_condition = vec![i as isize, j as isize, 0, 0];
            }
        }
    }
    // println!("{:?}", grid);
    // println!("{:?}", start_condition);

    let mut queue:VecDeque<Vec<isize>> = VecDeque::new();
    let mut visited:HashSet<Vec<isize>>  = HashSet::new();
    queue.push_back(start_condition.clone());
    
    let mut ans: isize = -1;

    'outer: while 0 < queue.len(){
        // println!("queue : {:?}", queue);
        let condition = queue.pop_front().unwrap();
        

        for i in four_neighborhood.iter(){
            let x = (condition[0] + i[0]);
            let y = (condition[1] + i[1]);
            if x < 0 || y < 0 || h-1 < x as usize|| w-1 < y as usize{
                // println!("out of range");
                continue;
            }

            let u_x = x as usize;
            let u_y = y as usize;
            // println!("{}, {}, {:?}", u_x, u_y, grid[u_x][u_y]);
            match grid[u_x][u_y]{
                '.' => {
                    let state = vec![x, y, condition[2]];
                    if !visited.contains(&state) {
                        visited.insert(state);
                        queue.push_back(vec![x, y, condition[2], condition[3] + 1]);
                    }
                }
                '#' => {continue}
                'S' => {
                    let state = vec![x, y, condition[2]];
                    if !visited.contains(&state) {
                        visited.insert(state);
                        queue.push_back(vec![x, y, condition[2], condition[3] + 1]);
                    }
                }
                'G' => {
                    ans = condition[3]+1;
                    break 'outer;
                }
                'o' => {
                    if condition[2]%2 == 0{
                        let state = vec![x, y, condition[2]];
                        if !visited.contains(&state) {
                            visited.insert(state);
                            queue.push_back(vec![x, y, condition[2], condition[3] + 1]);
                        }
                    }
                }
                'x' => {
                    if condition[2]%2 == 1{
                        let state = vec![x, y, condition[2]];
                        if !visited.contains(&state) {
                            visited.insert(state);
                            queue.push_back(vec![x, y, condition[2], condition[3] + 1]);
                        }
                    }
                }
                '?' => {
                    let state = vec![x, y, (condition[2]+1)%2];
                    if !visited.contains(&state) {
                        visited.insert(state);
                        queue.push_back(vec![x, y, (condition[2]+1)%2, condition[3] + 1]);
                    }
                }
                _ => {}
            }
        }
    }

    println!("{}", ans)

}