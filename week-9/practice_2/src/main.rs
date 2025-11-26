use std::io::Read;


fn main() {

    let mut file = std::fs::File::open("welcome_message.txt").expect("Open Failed");
    let mut contents = String::new();
    file.read_to_string(& mut contents).expect("Read Failed");
    println!("{}",contents);
}
