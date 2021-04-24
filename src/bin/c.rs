#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
        t: [(usize, usize, usize); q],
    }
    for i in 0..q {
        if t[i].0 == 1 {
            let tmp_a = s[t[i].1 - 1];
            s[t[i].1 - 1] = s[t[i].2 - 1];
            s[t[i].2 - 1] = tmp_a;
        } else if t[i].0 == 2 {
            let mut tmp: Vec<char> = s.drain(n..).collect();
            s.append(&mut tmp);
        }
    }
    for j in 0..(2 * n) {
        print!("{}", s[j]);
    }
}
