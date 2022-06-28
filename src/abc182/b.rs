use proconio::input;

fn main() {
    input! {
        a: [i32],
    }
    let max_a: i32 = *a.iter().max().unwrap();
    let mut gcd: i32 = 0;
    let mut ans: i32 = max_a;
    for p in 2..=max_a {
        let n = a.iter().filter(|&x| x % p == 0).sum();
        if n > gcd {
            ans = p;
            gcd = n;
        }
    }
    println!("{}", ans);
}
