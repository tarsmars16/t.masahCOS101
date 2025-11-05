//Rust Program
use std::io;
fn main() {
    loop {
        println!("-----Student Loan Repayment Estimator-----");
        println!("--Input the Values Below--");
        
        //creates variable to store user input
        let mut p_input = String::new();
        let mut r_input = String::new();
        let mut t_input = String::new();

        // principal
        println!("Enter the Value of Principal;");
        io::stdin().read_line(& mut p_input).expect("Failed to Read String");
        let principal: f32 = p_input.trim().parse().expect("Failed to Read Number");

        //rate
        println!("Enter the Value of Rate; ");
        io::stdin().read_line(& mut r_input).expect("Failed to Read String");
        let rate: f32 = r_input.trim().parse().expect("Failed to Reaf Number");

        //time
        println!("Enter the Number of Years; ");
        io::stdin().read_line(& mut t_input).expect("Failed to Read String");
        let time: f32 = t_input.trim().parse().expect("Failed to Read String");

        // formula to calculate amount and local interest
        let amount = principal * (1.0 + rate/100.0).powf(time);
        let li = amount - principal;

        //prints results
        println!("========================================================================================");
        println!("The Amount accumulated after the given amount of years is N{:.2}",amount);
        println!("The Local Interest is N{:.2}",li);
        println!("========================================================================================");


        println!("Would You like to calculate for another Student Borrower? (y/n) ");
        let mut decision = String::new();
        io::stdin().read_line(& mut decision).expect("Failed to Read Input");
        let decision = decision.trim().to_lowercase();

        // this is where the user makes decision to stop or continue loop
        if decision == "y" {
            println!("\nEnter the Information for the Next Student Borrower");
            continue;
        }
        else if decision == "n" {
            println!("\nThank you for using the Student Loan Repayment Estimator");
            break;
        }
        else { 
            println!("Please Enter a Vaild Decision, Either y to Continue or n to Stop");
            break;
        }
    }
}
