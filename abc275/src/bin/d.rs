use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut map: HashMap<usize, usize> = HashMap::new();
    let ans = f(n, &mut map);
    println!("{}", ans);
}

fn f(x: usize, map: &mut HashMap<usize, usize>) -> usize {
    if x == 0 {
        1
    } else {
        let f2 = if map.contains_key(&(x / 2)) {
            *(map.get(&(x / 2)).unwrap())
        } else {
            f(x / 2, map)
        };

        let f3 = if map.contains_key(&(x / 3)) {
            *(map.get(&(x / 3)).unwrap())
        } else {
            f(x / 3, map)
        };
        
        let res = f2 + f3;
        map.insert(x, res);
        res
    }
}