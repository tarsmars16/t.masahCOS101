use std::io::Write;
use std::fs::File;

fn main() {
    // creating vectors for the drink categories
    let lager = vec!["33 Export","Desperados","Goldberg","Gulder","Heineken","Star"];
    let stout = vec!["Legend","Turbo King","Williams"];
    let non_alcoholic = vec!["Maltina","Amstel Malta","Malta Gold","Fayrouz"];


    let mut file = File::create("Nigerian Brewery Drinks").expect("Create File Failed"); //creating the file

    file.write_all("----Nigerian Breweries Plc----".as_bytes()).expect("Write Failed"); //writing the title of the file

    file.write_all("\nLager:".as_bytes()).expect("Write Failed");
        for drinks in &lager {  //loops through each drink in the lager category
            writeln!(file, "\n - {}", drinks).expect("Failed"); 
        }
    file.write_all("\nStout:".as_bytes()).expect("Write Failed");
        for drinks in &stout { ////loops through each drink in the stout category
           writeln!(file, "\n - {}", drinks).expect("Failed");
        }
    file.write_all("\nNon-Alcoholic".as_bytes()).expect("Write Failed");
        for drinks in &non_alcoholic {  //loops through each drink in the non-alcoholic category
            writeln!(file, "\n - {}", drinks).expect("Failed");
        }
    println!("Success");

}
