use std::io;

pub fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let a = a.trim().parse::<i32>().unwrap();
    let mut b = String::new();
    io::stdin().read_line(&mut b).unwrap();
    let b = b.trim().parse::<i32>().unwrap();
    let mut c = String::new();
    io::stdin().read_line(&mut c).unwrap();
    let c = c.trim().parse::<i32>().unwrap();
    let total_score = a + b + c;

    if total_score >= 80 && total_score <= 100 {
        println!("A");
    } else if total_score >= 75 {
        println!("B+");
    } else if total_score >= 70 {
        println!("B");
    } else if total_score >= 65 {
        println!("C+");
    } else if total_score >= 60 {
        println!("C");
    } else if total_score >= 55 {
        println!("D+");
    } else if total_score >= 50 {
        println!("D");
    } else {
        println!("F");
    }
}
