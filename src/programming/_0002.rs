use std::io;

pub fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let mut n = n.trim().parse::<i32>().unwrap();
    let mut min = 2 * 1_000_000_000 + 1;
    let mut max = -2 * 1_000_000_000 - 1;
    while n > 0 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().parse::<i32>().unwrap();
        if input < min {
            min = input;
        }
        if input > max {
            max = input
        }
        n = n - 1;
    }
    println!("{}", min);
    println!("{}", max);
}
