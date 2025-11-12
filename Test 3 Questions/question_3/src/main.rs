//Rust program
use std::io;
// prints the cafe menu
fn main() {
    loop {

        println!("Welcome To The Cafe");
        println!("      Code            Item            Price(N)       ");
        println!("       T              Tea              800           ");
        println!("       C             Coffee            1,200         ");
        println!("       S            Sandwich           2,000         ");
        println!("       J             Juice             1,500         ");

        //stores the user input in a variable
        let mut input1 = String::new();
        let mut input2 = String::new();

        println!("Input an Item Code ( T / C / S / J ) ");
        io::stdin().read_line(& mut input1).expect("Failed to Read String");
        let itemcode = input1.trim().to_uppercase();

        println!("Input a Quantity");
        io::stdin().read_line(& mut input2).expect("Failed to Read String");
        let qty: f32 = input2.trim().parse().expect("Failed to Read Number");

        //the ..._alt formulas are for when total cost 5000
        let t = 800.0 * qty;
        let t_alt = 800.0 * qty * (0.95);
        let c = 1200.0 * qty;
        let c_alt = 1200.0 * qty * (0.95);
        let s = 2000.0 * qty;
        let s_alt = 2000.0 * qty * (0.95);
        let j = 1500.0 * qty;
        let j_alt = 1500.0 * qty * (0.95);

        // These else if function determine which is to be printed
        if itemcode == "T" {
           if t > 5000.0 { println!("Your Total Cost is N{:.2}",t);
                           println!("However Your Final Cost is N{:.2}",t_alt); 
                           println!("With a Discount off of 5%");
            }
           else { println!("Your Total Cost is N{:.2}",t); }
        }
        else if itemcode == "C" {
            if c > 5000.0 { println!("Your Total Cost is N{:.2}",c);
                            println!("However Your Final Cost is N{:.2}",c_alt); 
                            println!("With a Discount off of 5%");
            }
            else { println!("Your Total Cost is N{:.2}",c); }
        }
        else if itemcode == "S" {
            if s > 5000.0 { println!("Your Total Cost is N{:.2}",s);
                            println!("However Your Final Cost is N{:.2}",s_alt); 
                            println!("With a Discount off of 5%");
            }
            else { println!("Your Total Cost is N{:.2}",s); }
        }
        else if itemcode == "J" {
            if j > 5000.0 { println!("Your Total Cost is N{:.2}",j);
                            println!("However Your Final Cost is N{:.2}",j_alt); 
                            println!("With a Discount off of 5%");
            }
            else { println!("Your Total Cost is N{:.2}",j); }
        }
        else { println!("Please Enter Valid Item Code ( T / C / S / J );") }

        //asks the user if they want to create another entry
        println!("===============================================================================");
        println!("Do you wish to calculate Total Cost Again?");
        println!("If you do, click any key");
        println!("If you don't, type exit");
        println!("===============================================================================");
        let mut choice = String::new();
        io::stdin().read_line(& mut choice).expect("Failed to Read String");
        let choice = choice.trim().to_lowercase();
        // makes choice to stop or continue loop
        if choice == "exit" {
            println!("\nThank you for using this Cafe Program");
            break;
        }
        else {
            println!("\nCreating Another Entry...");
            continue;
        }
    }   

}

