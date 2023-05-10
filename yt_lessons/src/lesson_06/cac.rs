use std::io;

pub fn cac(){
    let _x = 251_f32;
    let y = 120_000i64;
    let z = 313 as f64;

    let o = y + z as i64;
    println!("{}", o);

    let t = (i8::MAX as i16) + 1;
    println!("{}", t as i8);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");

    let int_input: i64 = input.trim().parse().unwrap();
    
    println!("{}", int_input + 2)
}