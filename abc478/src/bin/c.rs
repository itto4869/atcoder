use ac_library::{Segtree, Max, Min};
use cp_library::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut l_sorted = vec![false; n];
    let mut r_sorted = vec![false; n];
    let mut max_seg = Segtree::<Max<usize>>::new(n);
    let mut min_seg = Segtree::<Min<usize>>::new(n);

    let mut prev = 0;
    for i in 0..n {
        if a[i] >= prev {
            l_sorted[i] = true;
            prev = a[i];
        } else {
            break;
        }
    }

    prev = usize::MAX;
    for i in (0..n).rev() {
        if a[i] <= prev {
            r_sorted[i] = true;
            prev = a[i];
        } else {
            break;
        }
    }

    for i in 0..n {
        max_seg.set(i, a[i]);
        min_seg.set(i, a[i]);
    }

    let mut ok = false;
    for i in 0..=(n - k) {
        if i == 0 {
            if r_sorted[i + k] && (max_seg.prod(i..(i + k)) <= min_seg.prod((i + k)..n)) {
                ok = true;
                break;
            }
        } else if i == (n - k) {
            if l_sorted[i - 1] && (min_seg.prod(i..(i + k)) >= max_seg.prod(0..i)) {
                ok = true;
                break;
            }
        } else {
            if l_sorted[i - 1] && r_sorted[i + k] && (max_seg.prod(i..(i + k)) <= min_seg.prod((i + k)..n)) && (min_seg.prod(i..(i + k)) >= max_seg.prod(0..i)) {
                ok = true;
                break;
            }
        }
    }

    yes_no!(ok);
}
