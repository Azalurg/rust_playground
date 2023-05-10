use std::io;

pub fn input() {
    let mut x = String::new();

    io::stdin().read_line(&mut x).expect("failed to read line");
    println!("{}", x);
}