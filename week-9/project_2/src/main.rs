use std::fs::File;
use std::io::Write;

fn main() {
    
    //creating columns as vectors 
    let student_name = vec!["Oluchi Mordi","Adams Aliyu","Shania Bolade","Adekunle Gold","Blanca Edemoh"];
    let matric_number = vec!["ACC10211111","ECO10110101","CSC10328828","EEE11020202","MEE10202001"];
    let department = vec!["Accounting","Economics","Computer","Electrical","Mechanical"];
    let level = vec![300,100,200,200,100];

    let mut file = File::create("student data.csv").expect("Create Failed"); //creating an csv file
    file.write_all("PAU SMIS".as_bytes()).expect("Write Failed"); //writing the header into the file
    file.write_all("\nStudent Name,Matric. Number,Department,Level\n".as_bytes()).expect("Write Failed");

    //the maximum number of rows of the table is number of items in the student_name vector
    let max_rows = student_name.len();

    for i in 0..max_rows { //loop from zero to the max_row value

        //get each value through index and places them as rows under the columns
        let column1 = student_name.get(i).expect("Write Failed");
        let column2 = matric_number.get(i).expect("Write Failed");
        let column3 = department.get(i).expect("Write Failed");
        let column4 = level.get(i).expect("Write Failed");

        writeln!(file, "{},{},{},{}", column1, column2, column3, column4).expect("Write Failed");
    }

    println!("Write Success");

}
