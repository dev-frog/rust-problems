use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    let m: usize = lines.next().unwrap().trim().parse().unwrap();
    let a: usize = lines.next().unwrap().trim().parse().unwrap();

    let mut x = n / a;
    let mut y = m / a;

    if n % a != 0 {
        x += 1;
    }

    if m % a != 0 {
        y += 1;
    }

    println!("{}", x * y);
}
