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

    let mut grid:Vec<Vec<char>> = vec![Vec::new();h];

    let mut queue:VecDeque<(usize, usize)> = VecDeque::new();

    for (row, s) in a.iter().enumerate(){
        for (col, c)  in s.chars().enumerate(){
            if c == '.'{
                queue.push_back((row, col));
            }
            grid[row].push(c);
        }
    }

    // println!("{:?}", grid);

    let mut cnt:usize = 0;
    let mut visited: Vec<Vec<bool>> = vec![vec![false; w]; h];

    while !queue.is_empty(){
        let (row, col) = queue.pop_front().unwrap();

        if visited[row][col] {  // ← これを追加
        continue;
    }

        let is_ok = recur(&mut visited, row, col, &grid, h, w);

        if is_ok {
            cnt += 1;
        }
    }

    println!("{}", cnt);

   
}

fn recur(visited:&mut Vec<Vec<bool>>, row:usize, col: usize, grid: &Vec<Vec<char>>, h:usize, w:usize) -> bool{
    if visited[row][col]{
        return true;
    }

    visited[row][col] = true;

    let on_border = row == 0 || row == h - 1 || col == 0 || col == w - 1;
    let mut is_ok = !on_border;

    for i in four_neighborhood {
        let nex_row = row as isize + i[0];
        let nex_col = col as isize + i[1];

        if nex_row < 0 || h as isize <= nex_row || nex_col < 0 || w as isize <= nex_col {
            continue;
        }

        let nex_row = nex_row as usize;
        let nex_col = nex_col as usize;

        if grid[nex_row][nex_col] == '#' {
            continue;
        }

        is_ok &= recur(visited, nex_row, nex_col, grid, h, w);
    }

    is_ok

}