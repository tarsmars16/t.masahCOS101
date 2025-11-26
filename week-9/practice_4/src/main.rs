use std::fs::OpenOptions;
use std::io::Write;
use std::io;

fn main() {
    
    let mut file = OpenOptions::new().append(true).open("file/data.txt").expect("Cannot Open File");
    file.write_all("\nHello Class".as_bytes()).expect("Write Failed");
    file.write_all("\nThis is the appendage to the document.".as_bytes()).expect("Write Failed");

    println!("Enter Information");
    let mut userinput = String::new();
    io::stdin().read_line(& mut userinput).expect("Failed to Read");
    let userinput = userinput.trim().to_string().;
    file.write_all(&userinput.into_bytes()).expect("Append Failed");

    println!("File append Success");
}
