#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    if a * a + b * b < c * c {
        println!("Yes");
    } else {
        println!("No");
    }
}
