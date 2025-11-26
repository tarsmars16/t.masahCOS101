use std::io; //interface for reading and writing input and output
fn main() {

    println!("==================================================================");
    println!("          -----PUBLIC SERVICE APS LEVEL CHECKER-----");
    println!("==================================================================");
    println!("               ------By Tamaratare Masah-----");
    println!();

    loop{
    println!("Please Select your Department from the List Below;");

    let dept = vec!["Office Administrator","Academic","Lawyer","Teacher"];

    for i in 0..dept.len() {
    println!("{}. {}", i + 1,dept[i]) }//this itemizes the departments for clarity using index

    let mut input1 = String::new();
    println!("\nInput the Number of the Corresponding Department ( 1,2,3,4 )");
    io::stdin().read_line(& mut input1).expect("Failed to Read Input");
    let deptinput:usize = input1.trim().parse().expect("Invalid Input"); //user input for department



    if deptinput == 1 { 
        let role = office_admin();//calls on office_admin function to return value
        let (experience, aps) = aps_level();//calls on aps_level function to return value
        println!("=================================");
        println!("\n----This is Your Information---");
        println!("Department: Office Administrator");
        println!("Role: {}",role);
        println!("Experience: {}",experience);
        println!("APS Level: {}", aps);
        println!("=================================");

    }
    else if deptinput == 2 {
        let role = academic();//calls on academic function to return value
        let (experience, aps) = aps_level(); //calls on aps_level function to return value
        println!("=================================");
        println!("\n---This is Your Information---");
        println!("Department: Academic");
        println!("Role: {}",role);
        println!("Experience: {}",experience);
        println!("APS Level: {}", aps);
        println!("=================================");
    }
    else if deptinput == 3 {
       let role = lawyer();//calls on lawyer function to return value
        let (experience, aps) = aps_level();//calls on aps_level function to return value
        println!("=================================");
        println!("\n---This is Your Information---");
        println!("Department: Lawyer");
        println!("Role: {}",role);
        println!("Experience: {}",experience);
        println!("APS Level: {}", aps);
        println!("=================================");

    }
    else if deptinput == 4 {
       let role = teacher();//calls on teacher function to return value
        let (experience, aps) = aps_level();//calls on aps_level function to return value
        println!("=================================");
        println!("\n---This is Your Information---");
        println!("Department: Teacher");
        println!("Role: {}",role);
        println!("Experience: {}",experience);
        println!("APS Level: {}", aps);
        println!("=================================");

    }
    else { println!("Please enter a Valid Department Number");
            break; } //stops code with invalid input

    println!("Would You Like to Check the APS Level of Another Public servant? (y / n)");
        // Ask if user wants to continue
    let mut choice = String::new();
    io::stdin().read_line(& mut choice).expect("Failed to Read Input");
    let choice = choice.trim().to_lowercase();//user input to continue or stop code

    if choice == "y" {
        println!("\nRerunning Program...");
        continue;//continues code
    }
    else if choice == "n" {
        println!("\nThank you for using the Public Servant APS Level Checker");
        break;
    }
    else { println!("Invalid Input"); 
            break; }//stops code
    }
}

fn aps_level() -> (f32,String) { //returns value as f32 and String for Experience and APSlevel respectively
    println!("===================================================");
    println!("What is your years of work experience?");
    println!("===================================================");
    let mut apsinput = String::new();
    io::stdin().read_line(& mut apsinput).expect("Failed to Read Input");
    let experience:f32 = apsinput.trim().parse().expect("Invalid Input");//user input for experience


    let apslevel = if experience >= 1.0 && experience <= 2.9 {
        "APS 1-2"
    }
    else if experience >= 3.0 && experience <= 4.9 {
        "APS 3-4"
    }
    else if experience >= 5.0 && experience <= 7.9 {
        "APS 5-8"
    }
    else if experience >= 8.0 && experience <= 9.9 {
        "APS 8-10"
    }
    else if experience >= 10.0 && experience <= 13.0 {
        "APS 10-13"
    }
    else {
      "SES"

    };

    (experience, apslevel.to_string())//converts to string
}

fn office_admin() -> String { //returns as string

    println!("------What is your Role in the Department?------");

    let role = vec!["Intern","Administrator","Senior Administrator","Office Manager","Director","CEO"];

    for i in 0..role.len() {
        println!("{}. {}", i,role[i]); //itemizes the roles of the office administrator department
    }

    println!("\nType in the Corresponding Number (Index Value) of Your Role from the listed above");
    let mut input2 = String::new();
    io::stdin().read_line(& mut input2).expect("Failed to Read Input");
    let roleinput:usize = input2.trim().parse().expect("Invalid Input"); //user input for role

    let roleoutput = role[roleinput];
    println!("Role : {}",roleoutput);
    return roleoutput.to_string();

}

fn academic() -> String { //returns as String
     println!("------What is your Role in the Department?------");

    let role = vec!["-","Research Assistant","PhD Candidate","Post-Doc Researcher","Senior Lecturer","Dean"];

    for i in 0..role.len() {
        println!("{}. {}", i,role[i]); //itemizes the roles of the academic department for clarity
    }

    println!("\nType in the Corresponding Number (Index Value) of Your Role from the listed above");
    let mut input2 = String::new();
    io::stdin().read_line(& mut input2).expect("Failed to Read Input");
    let roleinput:usize = input2.trim().parse().expect("Invalid Input"); //user input for role number

    let roleoutput = role[roleinput];
    println!("Role : {}",roleoutput);
    return roleoutput.to_string();
}

fn lawyer() -> String { //returns as String
     println!("-----What is your Role in the Department?-----");

    let role = vec!["Paralegal","Junior Associate","Associate","Senior Associate 1-2","Senior Associate 3-4","Partner"];

    for i in 0..role.len() {
        println!("{}. {}", i,role[i]); //itemizes the roles of the Lawyer Department
    }

    println!("\nType in the Corresponding Number (Index Value) of Your Role from the listed above");
    let mut input2 = String::new();
    io::stdin().read_line(& mut input2).expect("Failed to Read Input");
    let roleinput:usize = input2.trim().parse().expect("Invalid Input"); //user input for role number

    let roleoutput = role[roleinput];
    println!("Role : {}",roleoutput);
    return roleoutput.to_string();
}

fn teacher() -> String { //returns as String
     println!("-----What is your Role in the Department?-----");

    let role = vec!["Placement","Classroom Teacher","Snr Teacher","Leading Teacher","Deputy Principal","Principal"];

    for i in 0..role.len() {
        println!("{}. {}", i,role[i]); //itemizes the roles of the Teacher Department
    }

    println!("\nType in the Corresponding Number (Index Value) of Your Role from the listed above");
    let mut input2 = String::new();
    io::stdin().read_line(& mut input2).expect("Failed to Read Input");
    let roleinput:usize = input2.trim().parse().expect("Invalid Input"); //user input for role number

    let roleoutput = role[roleinput];
    println!("Role : {}",roleoutput);
    return roleoutput.to_string(); 
}