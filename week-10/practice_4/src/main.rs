fn main() {
    // a list of nos
    let v = vec![15,25,35,45,55];
    print_vector(v.clone());
    println!("{}",v[0]); //this line gives an error without the .clone() to make a copy of the value
}

fn print_vector(x:Vec<i32>) {
    
    println!("Inside print_vector function {:?}",x);
}