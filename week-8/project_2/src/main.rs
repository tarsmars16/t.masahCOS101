use std::io;
fn main() {
    let mut developers: Vec<(String, u32)> = Vec::new();

    println!("==============--Ernst & Young (EY) Global Limited--==============");
    println!("              === EY Nigeria Experience Checker ===");

    loop {
        println!("\nEnter The Name of the Developer:");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Failed to Read String");
        let name = input1.trim().to_string(); //user input for the name of the developer

        let mut input2 = String::new();
        println!("\nEnter years of experience for {}:", name);
        io::stdin().read_line(&mut input2).expect("Failed to Read String");
        let experience: u32 = input2.trim().parse().expect("Invalid Input"); 
        
    
        developers.push((name, experience));

        // Ask if user wants to continue
        let mut input3 = String::new();
        println!("\nDo you want to enter another developer? (y/n):");
        io::stdin().read_line(&mut input3).expect("Failed to Read String");
        let choice = input3.trim().to_lowercase();

        if choice == "n" {
            println!("\nThank you for using EY Nigeria Experience Checker");
            break; 
        }
        else if choice == "y" {
            println!("\nInput the Information of the Next Developer\n");
        }
        else { println!("Invalid Input. Try Again");
        continue; }

    }
    //stops if no developers were entered
    if developers.is_empty() {
        println!("No developers entered.");
        return;
    }

    let mut max_index = 0; //representing the index of the developer with the highest experience.

    for i in 1..developers.len() { //Loop through all other developers
        if developers[i].1 > developers[max_index].1 { //checks if experience is higher the current
        max_index = i; } 

    }

    let highest_dev = &developers[max_index]; //stored in a variable

    println!("Most experienced developer: {} with {} years", highest_dev.0, highest_dev.1);

    //Results

    println!("\n======== RESULT==========");
    println!("Most experienced developer:");
    println!("Name: {}", highest_dev.0);
    println!("Experience: {} years", highest_dev.1);
}
