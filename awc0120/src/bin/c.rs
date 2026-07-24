use std::collections::VecDeque;

use cp_library::yes_no;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut graph = vec![Vec::new(); n];
    for i in 0..(n - 1) {
        input! {
            p: Usize1,
        }
        graph[p].push(i + 1);
    }
    
    input! {
        v: [usize; n],
    }

    let mut ok = true;
    let mut queue = VecDeque::new();
    queue.push_back(0);
    while let Some(u) = queue.pop_front() {
        let next = &graph[u];
        let mut sum = 0;
        for &nu in next {
            if v[nu] > v[u] {
                ok = false;
            } else {
                sum += v[nu];
                queue.push_back(nu);
            }
        }

        if sum > v[u] {
            ok = false;
            break;
        }
    }

    yes_no!(ok);
}