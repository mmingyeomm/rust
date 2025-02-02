use std::fs::File;
use std::io::Error; 



fn main() {
    let f: Result<File, Error> = File::open("hello.txt");


    match f {
        Ok(_) => {},
        Err(_) => {},
    }


    
}
