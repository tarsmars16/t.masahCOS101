//Rust Program
use std::io;
fn main() {
    println!("-----Monthly Electricity Bill Evaluator-----");

    //Creates a variable to store user input
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("What is Your name?");
    io::stdin().read_line(& mut input1).expect("Failed to Read Input");
    let name = input1.trim();

    println!("Input the number of units consumed");
    io::stdin().read_line(& mut input2).expect("Failed to Read Input");
    let units: f32 = input2.trim().parse().expect("Failed to Read Number");

   if units <= 100.0 && units >= 0.0 {
    let rate: f32 = 20.0;
    let totalunits = units * rate as f32;
    println!("\n---- Welcome {} -----",name);
    println!("\nYou have consumed {:.2} units",units);
    println!("\nThe rate of your power consumption is {}kWh",rate);
    println!("\nYour total bill is N{:.2}",totalunits);
   }
   else if units <= 300.0 && units >= 101.0 {
    let rate: f32 = 35.0;
    let totalunits = units * rate as f32;
    println!("\n---- Welcome {} -----",name);
    println!("\nYou have consumed {:.2} units",units);
    println!("\nThe rate of your power consumption is {}kWh",rate);
    println!("\nYour total bill is N{:.2}",totalunits);
   }
   else if units <= 301.0 && units >= 500.0 {
    let rate: f32 = 50.0;
    let totalunits = units * rate as f32;
    println!("\n---- Welcome {} -----",name);
    println!("\nYou have consumed {:.2} units",units);
    println!("\nThe rate of your power consumption is {}kWh",rate);
    println!("\nYour total bill is N{:.2}",totalunits);
   }
   else if units > 500.0 {
    let rate = 50.0;
    let totalunits = (units * rate) + 5000.0 as f32;
    println!("\n---- Welcome {} -----",name);
    println!("\nYou have consumed {:.2} units",units);
    println!("\nThe rate of your power consumption is {}kWh",rate);
    println!("\nYour total bill is N{:.2} with a charge of N5,000.00",totalunits);
   }
    
}
