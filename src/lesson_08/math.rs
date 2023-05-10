pub fn math(x: f64, y: f64, operation: &str) -> f64 {
    if operation == "add" {
        return add(x, y);
    } else if operation == "sub" {
        return sub(x, y)
    } else if operation == "mul" {
        return mul(x, y)
    } else if operation == "div" {
        return div(x, y)
    }
    println!("Operation not found: {}", operation);
    -1_f64
}

fn add(x: f64, y: f64) -> f64 {
    x+y
}

fn sub(x: f64, y: f64) -> f64 {
    x-y
}

fn mul(x: f64, y: f64) -> f64 {
    x*y
}

fn div(x: f64, y: f64) -> f64 {
    x/y
}