fn main() {
    let tup = ("Ala", 11);
    let numbers = [1, 2, 3, 4, 5];

    get_personal_data(tup);
    println!("{}", sum(numbers));

// ---

    println!("{}", if true {"ma"} else {"kota"});

// ---

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter;
        }
    };

// ---

    for n in 1..5{
        println!("value: {}", n)
    }
}

fn get_personal_data(tup: (&str, i32)) {
    println!("{} ma {} lat", tup.0, tup.1);
}

fn sum(numbers: [i32; 5]) -> i32{
    let mut suma = 0;
    for n in numbers {
        suma = suma + n;
    }
    return suma;
}