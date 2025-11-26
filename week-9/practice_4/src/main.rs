use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    
    let mut file = OpenOptions::new().append(true).open("file/data.txt").expect("Cannot Open File");
    file.write_all("\nHello Class".as_bytes()).expect("Write Failed");
    file.write_all("\nThis is the appendage to the document.".as_bytes()).expect("Write Failed");

    println!("File append Success");
}
