use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let _file = open_file("data.txt");

}

fn open_file(path: &str) -> File{
    let f = File::open(path);
    match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e)
            },
            other_error => panic!("Problem opening the file: {:?}", other_error)
        }
    }
}
