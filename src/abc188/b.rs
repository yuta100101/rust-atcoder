use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
        b: [i32; n],
    }
    if a.iter().zip(b.iter()).map(|(x, y)| x * y).sum::<i32>() == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
