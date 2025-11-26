use std::fs;

fn main() {
   fs::remove_file("data.txt").expect("Could not Remove file");
   println!("File is Removed!");
}
