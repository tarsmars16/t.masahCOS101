use std::io;
fn main() {
    loop {
        println!();
        println!("               Welcome to Tenderness Kitchen           ");
        println!();
        println!("    Please help yourself with our tasty meals available  ");
        println!();        
        println!("_______________________________________________________________");
        println!("|          Menu                            Price              |");
        println!("| P = Poundo Yam/Edinkaiko Soup              -N3,200          |");
        println!("| F = Fried Rice & Chicken                   -N3,000          |");
        println!("| A = Amala & Ewedu Soup                     -N2,500          |");
        println!("| E = Eba & Ewedu Soup                       -2,000           |");
        println!("| W = White Rice & Stew                      -2,500           |");
        println!("|_____________________________________________________________|");

    
        let mut input1 = String::new();
        let mut input2 = String::new();
        let mut price: f32 = 0.0;
        let mut item_name: &str = "";

        println!("\nWhat would you like from the menu? ( P / F / A / E / W )");
        io::stdin().read_line(& mut input1).expect("Failed to Read String");
        let choice = input1.trim().to_uppercase();


        if choice == "P" {
            price = 3200.0;
            item_name = "Pounded Yam with Edinkaiko Soup";
        }
        else if choice == "F" {
            price = 3000.0;
            item_name = "Fried Rice & Chicken";
        }
        else if choice == "A" {
            price = 2500.0;
            item_name = "Amala & Ewedu Soup";
        }

        else if choice == "E" {
            price = 2000.0;
            item_name = "White Rice & Stew";
        }
        else if choice == "W" {
            price = 2500.0;
            item_name = "White Rice & Stew";
        }
        else {
            println!("Please enter a vaild order label ( P / F / A / E / W )");
            break;
        }

        println!("\nHow many portions of {} would you like?",item_name);
        io::stdin().read_line(& mut input2).expect("Failed to Read String");
        let portion: f32 = input2.trim().parse().expect("Failed to Read Number");

        let total_price = price * portion;
        let total_price_alt = price * portion * 0.95;

        if total_price > 10000.0 {
        println!("\nThe Total Price of your Order will be N{:.2}",total_price_alt);
        println!("With a Discount of 5%");
        }
        else { 
            println!("\nThe Total Price of your Order will be N{:.2}",total_price_alt);
        }

        let mut input3 = String::new();
        println!("\n======================================================================================");
        println!("Would you Like to make another tasty order? (y/n)");
        println!("======================================================================================");
        io::stdin().read_line(& mut input3).expect("Failed to Read String");
        let decision = input3.trim().to_lowercase();

        if decision == "y" {
            println!("\n         --We are delighted to serve you again!--");
            println!("\n          --What else would you like to order?--");
            continue;
        }
        else if decision == "n" {
            println!("\nThanks for Ordering at Tenderness Kitchen!");
            println!("Enjoy your meal");
            break;
        }
        else {
            println!("Please indicate either y to order another meal or n to stop");
            break;
        }

    }
}

 




