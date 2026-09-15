use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Usize1,
        l: usize,
        a: [usize; n - 1],
    }
    let mut v = vec![0; n];
    for i in 1..n {
        v[i] = v[i - 1] + a[i - 1];
    }

    let mut ans = 0;
    for i in (0..s).rev() {
        let l_d = v[s].abs_diff(v[i]);
        if l_d > l {
            break;
        }

        let res = l - l_d;
        let r_i = v.partition_point(|&x| x <= (v[i] + res)) - 1;

        ans = ans.max(s - i + r_i.saturating_sub(s) + 1);
    }

    for i in s..n {
        let r_d = v[s].abs_diff(v[i]);
        if r_d > l {
            break;
        }

        let res = l - r_d;
        let l_i = v.partition_point(|&x| x < v[i].saturating_sub(res));

        ans = ans.max(i - s + s.saturating_sub(l_i) + 1);
    }

    println!("{}", ans);
}
