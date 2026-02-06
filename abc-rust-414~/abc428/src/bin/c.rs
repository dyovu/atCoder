// use proconio::input;
use std::io::{self, BufRead};
use std::collections::*;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let q = lines.next().unwrap().unwrap().parse::<usize>().unwrap();

    let mut deq:VecDeque<char> = VecDeque::new();
    let mut m_range:isize = 0;
    let mut rem = 0;

    for i in lines.take(q){
        let query = i.unwrap().split_whitespace().map(|x| x.chars().next().unwrap()).collect::<Vec<char>>();
        let query_type = query[0].to_digit(10).unwrap();

        match query_type{
            1 => {
                deq.push_back(query[1]);
                if query[1] == '('{
                    m_range += 1;
                }else{
                    m_range -= 1;
                    if m_range < 0{
                        rem += 1;
                    }
                }
            }
            2 => {
                let k = deq.pop_back().unwrap();
                if k == '('{
                    m_range -= 1;
                }else{
                    if m_range < 0{
                        rem -= 1;
                    }
                    m_range += 1;
                    
                }
            }
            _ => {}
        }
        // println!("{:?}", deq);

        if m_range == 0 && rem == 0{
            println!("Yes");
        }else{
            println!("No");
        }
    }


}