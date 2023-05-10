pub fn conditions() {
    let cond = (2 as f32) < 3.2;
    println!("{}", cond);

    let cond2 = false || cond;
    println!("{}", cond2);

    let food = "cookie";
    if food == "cookie" {
        println!("it is cookie")
    }
}