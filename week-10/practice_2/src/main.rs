fn main() {
    let v = vec![10,20,30];
    //vector v owns the object in heap

    let v2 = v;

    display(v2.clone()); 
    //v2 is moved to display and v2 is invalidated
    //however with the .clone(), the copy of the value in v2 is instead being passed
    //not the v2 itself

    println!("In main {:?}",v2);
    //v2 is no longer usable here
}

fn display(v:Vec<i32>) {
    println!("Inside display {:?}",v);
}