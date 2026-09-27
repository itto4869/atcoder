use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _: usize,
        n1: usize,
        n2: usize,
        vl1: [(usize, usize); n1],
        vl2: [(usize, usize); n2],
    }
    let mut ans = 0;
    let mut v1 = Vec::new();
    let mut curr = 0;
    for &(v, l) in &vl1 {
        curr += l;
        v1.push((v, curr))
    }

    let mut v2 = Vec::new();
    curr = 0;
    for &(v, l) in &vl2 {
        curr += l;
        v2.push((v, curr));
    }

    curr = 0;
    let mut i = 0;
    let mut j = 0;
    while (i < n1) && (j < n2) {
        let (v1, l1) = v1[i];
        let (v2, l2) = v2[j];

        if v1 == v2 {
            ans += l1.min(l2) - curr;
        }

        if l1 < l2 {
            curr = l1;
            i += 1;
        } else if l1 > l2 {
            curr = l2;
            j += 1;
        } else {
            curr = l1;
            i += 1;
            j += 1;
        }
    }

    println!("{}", ans);
}
