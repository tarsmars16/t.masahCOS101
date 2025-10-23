use std::io;

fn main() {
    println!("\nFind the Roots of The Equation");
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();
    
    println!("\nEnter the first value a: ");
    io::stdin().read_line(& mut a).expect("Not a Valid String");
    let a: f32 = a.trim().parse().expect("Not a Valid Number");

    println!("\nEnter the second value b: ");
    io::stdin().read_line(& mut b).expect("Not a Valid String");
    let b: f32 = b.trim().parse().expect("Not a Valid Number");

    println!("\nEnter the second value c: ");
    io::stdin().read_line(& mut c).expect("Not a Valid String");
    let c: f32 = c.trim().parse().expect("Not a Valid Number");

    let discriminant = b.powf(2.0) - 4.0 * a * c;

    if discriminant > 0.0 {
        println!("Your Equation has two distinct roots");
    }
    else if discriminant == 0.0 {
        println!("Your Equation only one root");
    }
    else if discriminant < 0.0 {
        println!("Your Equation has no real roots")
    }
}
