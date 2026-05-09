use std::collections::HashMap;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut l_v = vec![Vec::new(); n];
    let mut r_v = vec![Vec::new(); n];
    let mut map = HashMap::new();
    let mut v1 = vec![n; n];
    let mut v2 = vec![0; n];
    for i in 0..m {
        input! {
            l: Usize1,
            r: Usize1,
        }
        l_v[l].push((r, i));
        r_v[r].push((l, i));
        v1[l] = v1[l].min(r);
        v2[r] = v2[r].max(l);
        *map.entry((l, r)).or_insert(0usize) += 1;
    }

    for i in (0..(n - 1)).rev() {
        v1[i] = v1[i].min(v1[i + 1]);
    }

    for i in 1..n {
        v2[i] = v2[i].max(v2[i - 1]);
    }
    
    input! {
        q: usize,
    }

    for i in 0..l_v.len() {
        l_v[i].sort_unstable();
        l_v[i].reverse();

        r_v[i].sort_unstable();
    }
    for _ in 0..q {
        input! {
            s: Usize1,
            t: Usize1,
        }
        if let Some(&cnt) = map.get(&(s, t)) {
            if cnt >= 2 {
                println!("Yes");
                continue;
            } else {
                if s + 1 < n && v1[s + 1] <= t {
                    println!("Yes");
                } else if t > 0 && v2[t - 1] >= s {
                    println!("Yes");
                } else {
                    println!("No");
                }
                continue;
                let s_v = &mut l_v[s];
                let t_v = &mut r_v[t];

                let s_i = s_v.partition_point(|&(x, _)| x > t);
                let t_i = t_v.partition_point(|&(x, _)| x < s);

                if s_i >= s_v.len() || t_i >= t_v.len() {
                    println!("No");
                    continue;
                }

                let (lx, l_i) = s_v[s_i];
                let (rx, r_i) = t_v[t_i];
                if l_i == r_i {
                    if (s_i + 1) < s_v.len() {
                        let s_i = s_i + 1;
                        let (x, _) = s_v[s_i];
                        if x + 1 >= t_v[t_i].0 {
                            println!("Yes");
                        } else {
                            println!("No");
                        }
                    } else if (t_i + 1) < t_v.len() {
                        let t_i = t_i + 1;
                        let (x, _) = t_v[t_i];
                        if s_v[s_i].0 + 1 >= x {
                            println!("Yes");
                        } else {
                            println!("No");
                        }
                    } else {
                        println!("No");
                    }
                } else {
                    if lx + 1 >= rx {
                        println!("Yes");
                    } else {
                        println!("No");
                    }
                }
            }
        } else {
            let s_v = &mut l_v[s];
            let t_v = &mut r_v[t];

            let s_i = s_v.partition_point(|&(x, _)| x > t);
            let t_i = t_v.partition_point(|&(x, _)| x < s);

            

            if s_i >= s_v.len() || t_i >= t_v.len() {
                println!("No");
                continue;
            }

            let (lx, l_i) = s_v[s_i];
            let (rx, r_i) = t_v[t_i];
            if lx + 1 >= rx {
                println!("Yes");
            } else {
                println!("No")
            }
        }
    }
}
