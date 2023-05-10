use std::io;

pub fn fizzbuzz() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("error");

    let number: i32 = line.trim().parse().unwrap();

    if number % 15 == 0 {
        println!("FizzBuzz")
    } else if number % 3 == 0 {
        println!("Fizz")
    } else if number % 5 == 0 {
        println!("Buzz")
    } else {
        println!("{}", number)
    }
}