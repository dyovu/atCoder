use proconio::input;
// use std::io::{self, BufRead};
use std::collections::*;
use std::rc::Rc;

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main(){
    input!{
        n: usize,
        a: [usize; n],
    }

    let seq = a.iter().map(|x| x-1).collect::<Vec<usize>>();
    // node -> (サーキット内の位置, サーキット本体)
    let mut node_info: HashMap<usize, (usize, Rc<Vec<usize>>)> = HashMap::new();
    let mut ans = Vec::new();

    for i in seq.iter(){
        let mut idx = *i;
        let mut v = vec![*i];
        let mut s = HashSet::new();
        let mut cnt = 0;
        s.insert(*i);

        'outer: loop{
            // 別のサーキットに入る
            if let Some((d, circuit)) = node_info.get(&idx).cloned() {
                let len = circuit.len();
                for (j, &node) in v.iter().enumerate() {
                    node_info.entry(node).or_insert_with(|| {
                        let stored_d = ((cnt as isize - j as isize - d as isize)
                            .rem_euclid(len as isize)) as usize;
                        (stored_d, Rc::clone(&circuit))
                    });
                }
                let slot = calc_slot(cnt + d, &circuit);
                ans.push(slot + 1);
                break 'outer;
            }
            
            let k = seq[idx];

            // 新しくサーキットが見つかる
            if s.contains(&k){
                let border = v.iter().position(|&x| x == k).unwrap();
                let new_circuit: Vec<usize> = v[border..].to_vec();
                let slot = calc_slot(border, &new_circuit);
                ans.push(slot + 1);

                let circuit = Rc::new(new_circuit);
                let len = circuit.len();
                for (pos, &node) in circuit.iter().enumerate() {
                    node_info.insert(node, (pos, Rc::clone(&circuit)));
                }
                // テールノードをキャッシュ
                for (j, &node) in v[..border].iter().enumerate() {
                    let stored_d = (border - j) % len;
                    node_info.entry(node).or_insert((stored_d, Rc::clone(&circuit)));
                }
                break;
            }  

            cnt +=1;
            idx = k;
            s.insert(k);
            v.push(k);
        }
    }

    for i in ans{
        print!("{} ", i);
    }
    
}


fn calc_slot(offset: usize, cir: &Vec<usize>) -> usize{
    let len = cir.len() as isize;
    let pow = mod_pow(10, 100, len);
    let idx = (pow + len - offset as isize % len) % len;
    cir[idx as usize]
}

fn mod_pow(mut base: isize, mut exp: isize, modulus: isize) -> isize {
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp /= 2;
        base = base * base % modulus;
    }
    result as isize
}