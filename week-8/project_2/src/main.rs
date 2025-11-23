use std::io;
fn main() {
    let mut developers: Vec<(String, u32)> = Vec::new();

    println!("==============--Ernst & Young (EY) Global Limited--==============");
    println!("              === EY Nigeria Experience Checker ===");

    loop {
        println!("Enter The Name of the Developer:");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Failed to Read String");
        let name = input1.trim().to_string(); //user input for the name of the developer

        let mut input2 = String::new();
        println!("Enter years of experience for {}:", name);
        io::stdin().read_line(&mut input2).expect("Failed to Read String");
        let experience: u32 = input2.trim().parse().expect("Invalid Input"); 
        
    
        developers.push((name, experience));

        // Ask if user wants to continue
        let mut input3 = String::new();
        println!("Do you want to enter another developer? (y/n):");
        io::stdin().read_line(&mut input3).expect("Failed to Read String");
        let choice = input3.trim().to_lowercase();

        if choice == "n" {
            println!("Thank you for using EY Nigeria Experience Checker");
            break; 
        }
        else if choice == "y" {
            println!("Below are the Results");
        }
        else { println!("Invalid Input. Try Again");
        continue; }

    }

    if developers.is_empty() {
        println!("No developers entered.");
        return;
    }

    let mut max_index = 0;

    for i in 1..developers.len() {
        if developers[i].1 > developers[max_index].1 {
        max_index = i; }

    }

    let max_dev = &developers[max_index];

    println!("Most experienced developer: {} with {} years", max_dev.0, max_dev.1);

    println!("\n======== RESULT==========");
    println!("Most experienced developer:");
    println!("Name: {}", max_dev.0);
    println!("Experience: {} years", max_dev.1);
}
