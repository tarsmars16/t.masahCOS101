use std::io;

fn add(a: i32, b: i32) {
    let sum = a + b;

    println!("Sum of A and B = {}", sum);

}
fn main () {
    let mut input1 = String::new();
    println!("Enger input for parameter A: ");
    io::stdin().read_line(& mut input1).expect("Failed to Read Input");
    let a: i32 = input1.trim().parse().expect("Invalid Input");

    let mut input2 = String::new();
    println!("Enter input for parameter B: ");
    io::stdin().read_line(& mut input2).expect("Failed to Read Input");
    let b: i32 = input2.trim().parse().expect("Invalic Input");

    //call add functions with arguments
    add(a, b);
    
}