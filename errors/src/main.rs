use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("Hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            },
        }
    };

    let _fil = File::open("hellso.txt").expect("Failed to open");
}


fn _last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
