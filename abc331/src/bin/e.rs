use ac_library::Segtree;
use ac_library::Max;
use proconio::marker::Usize1;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let mut v = vec![Vec::new(); n];
    for _ in 0..l {
        input! {
            c: Usize1,
            d: Usize1
        }
        v[c].push(d);
    }
    
    let mut segtree = Segtree::<Max<usize>>::new(m);
    for i in 0..m {
        segtree.set(i, b[i]);
    }

    let mut ans = 0;
    for i in 0..n {
        let ai = a[i];
        let ds = &v[i];
        for &di in ds {
            segtree.set(di, 0);
        }

        let max_bi = segtree.prod(0..m);
        ans = ans.max(ai + max_bi);

        for &di in ds {
            segtree.set(di, b[di]);
        }
    }

    println!("{}", ans);
}
