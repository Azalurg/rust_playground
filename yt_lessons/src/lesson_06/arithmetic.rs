pub fn arithmetic(){
    let x: u8 = 13;
    let y: u8 = 11;

    println!("u8 | x: {}, y: {}", x, y);
    println!("x + y = {}", x + y);
    println!("x - y = {}", x - y);
    println!("x * y = {}", x * y);
    println!("x / y = {}", x / y);
    println!("x mod y = {}\n", x % y);

    let x: f32 = 13.3;
    let y: f32 = 17.1;

    println!("f32 | x: {}, y: {}", x, y);
    println!("x + y = {}", x + y);
    println!("x - y = {}", x - y);
    println!("x * y = {}", x * y);
    println!("x / y = {}", x / y);
    println!("x mod y = {}\n", x % y);
}