use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
    }
    let mut v = vec![0; n];
    v[0] = a[0];
    for i in 1..n {
        v[i] = v[i - 1] + a[i];
    }

    let mut u = vec![0; n];
    u[0] = v[0];
    for i in 1..n {
        u[i] = u[i - 1] + v[i];
    }

    let mut idxes = vec![0; n];
    for i in 1..n {
        idxes[i] = idxes[i - 1] + i;
    }

    for _ in 0..q {
        input! {
            l: Usize1,
            r: Usize1,
        }
        let li = idxes.partition_point(|&x| x <= l);
        let lidx = l - idxes[li - 1];

        let ri = idxes.partition_point(|&x| x <= r);
        let ridx = r - idxes[ri - 1];

        if li == ri {
            if lidx == 0 {
                println!("{}", v[ridx]);
            } else {
                println!("{}", v[ridx] - v[lidx - 1]);
            }
            continue;
        }

        let mut res = u[ri - 1] - u[li - 1];

        if lidx == 0 {
            res += v[li - 1];
        } else {
            res += v[li - 1] - v[lidx - 1];
        }
        res -= v[ri - 1] - v[ridx];

        //println!("{} {} {} {} {}", res, li, lidx, ri, ridx);
        println!("{}", res);
    }
}
