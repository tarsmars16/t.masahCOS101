fn main() {
    //USing Vec::new()
    let vec1 : Vec<i64> = Vec::new();

    //printing the size of the vector
    println!("\nThe length of the Vec::new is {}",vec1.len());

    //Using macro
    let vec1 = vec!["Grace","Effiong","Basil","Kareem","Susan"];

    //printing the size of the vector
    println!("\nThe length of the vec macro is: {}",vec1.len());


}
