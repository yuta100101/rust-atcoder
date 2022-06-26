use proconio::input;

fn main() {
    input! {
        a: [i32; 4usize],
    }
    println!("{}", a.iter().min().unwrap());
}
