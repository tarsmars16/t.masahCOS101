use std::io;
fn main() {

   println!("\n Automatic Employee Incentive Determiner");
    let mut experience = String::new();
    let mut input = String::new();

   println!("\nAre you an Experienced Worker? (yes/no)");
   io::stdin().read_line(& mut experience).expect("Falied to read input");
   let experience = experience.trim();

   println!("\nEnter your age");
   io::stdin().read_line(& mut input).expect("Failed to read input");
   let age: u8 = input.trim().parse().expect("Please enter a vaild age");

   
    if experience == "yes"{
        if age >= 40 {
            let incentive = 1_560_000.00;
            println!("Your Incentive is ₦{:.2}", incentive);
        }
   
        if age >= 30 && age < 40 {
           let incentive = 1_480_000.00;
            println!("Your Incentive is ₦{:.2}", incentive);
        }
        if age < 28 {
            let incentive = 1_300_000.00;
            println!("Your incentive is ₦{:.2} per month", incentive);
        }
    }
    else if experience == "no" {
        let incentive = 100_000;
        println!("Your incentive is ₦{:.2}", incentive);
    }

}
