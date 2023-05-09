pub fn compound() {
    let mut tup: (i32, bool, char) = (7, true, 's');
    tup.0 = tup.0 * 2;
    tup = (11, false, ';');
    println!("{}", tup.1);

    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    arr[2] = arr[1] + arr[3];
    println!("{}", arr[2])
}