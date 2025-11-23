use std::io;
fn main() {

     println!("\nWhat is your Role in the Department?\n");

    let role = vec!["Intern","Administrator","Senior Administrator","Office Manager","Director","CEO"];

    for i in 0..role.len() {
        println!("{}. {}", i,role[i]);
    }

    println!("\nType in the Corresponding Number (Index Value) of Your Role from the listed above\n");
    let mut input2 = String::new();
    io::stdin().read_line(& mut input2).expect("Failed to Read Input");
    let roleinput:usize = input2.trim().parse().expect("Invalid Input");

    let roleoutput = role[roleinput];
    println!("Role : {}",roleoutput);

}