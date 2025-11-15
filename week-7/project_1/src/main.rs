use std::io;
fn main() {
    println!("\n-----Shape Area and Volume Calculator-----");
    println!("        ----By Tamaratare Masah----");
    println!();

    loop {
        println!("Would you like to calculate Volume or Area? ( V / A )");
        let mut input1 = String::new();
        io::stdin().read_line(& mut input1).expect("Failed to Read String");
        let vora = input1.trim().to_uppercase();

        if vora == "A" {
           println!("\nWhat Area of Shape would you like to calculate?");
           println!("Trapezium (T)   Rhombus (R)   Parallelogram (P)   Cube (C) ");

            let mut input2 = String::new();
            io::stdin().read_line(& mut input2).expect("Failed to Read String");
            let shape_a = input2.trim().to_uppercase();

            if shape_a == "T" {
                trapezium();        
            }
            if shape_a == "R" {
                rhombus();
            }
            if shape_a == "P" {
                parallelogram();
            }
            if shape_a == "C"{
                cube_a();
            }
            else { println!("Enter a Valid Shape Initial ( T / R / P / C )");
                  break; }
        }
        else if vora == "V" {
            println!("\nWhat Volume of Shape would you like to calculate?");
            println!("Cube (C)  Cylinder(K)");

            let mut input3 = String::new();
            io::stdin().read_line(& mut input3).expect("Failed to Read String");
            let shape_v = input3.trim().to_uppercase();

            if shape_v == "C" {
            cube_v();
            }
            if shape_v == "K" {
            cylinder();
            }   
            else { println!("Enter a Valid Shape Initial ( C / K )");
                  break; 
            }
        }
        
        else { println!();
           println!("Please Enter a Valid Choice");
           println!("Either V to calculate Volume or A to calulate Area");
           break;
        }

        println!("\n---Would you like to continue calculating? (y/n)---");
        let mut input4 = String::new();
        io::stdin().read_line(& mut input4).expect("Failed to Read String");
        let choice = input4.trim().to_lowercase();

        if choice == "y" {
            println!("\n---Rerunning Program...");
            continue;
        }
        else if choice == "n" {
            println!("/n---Thank you for using this Calculator---");
            break;
        }
        else { println!("\nNext time, enter either y to Continue of n to Stop");
            break; 
        }
    }
}


fn get_pi()-> f32 {
    let a = 22.0;
    let b = 7.0;
    let c: f32 = a / b;
    return c;
}
fn trapezium() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("\nInput the Height of the Trapezium;");
    io::stdin().read_line(& mut a).expect("Failed to Read String");
    let height: f32 = a.trim().parse().expect("Invalid Input");

    println!("\nInput the First Base of the Trapezium;");
    io::stdin().read_line(& mut b).expect("Failed to Read String");
    let base1: f32 = b.trim().parse().expect("Invalid String");

    println!("\nInput the Second Base of the Trapezium;");
    io::stdin().read_line(& mut c).expect("Failed to Read String");
    let base2: f32 = c.trim().parse().expect("Invalid Input");

    let area = height / 2.0 * (base1 + base2);
    println!("\n--The Area of The Trapezium is {:.2}cm²--",area);
}

fn rhombus() {
    let mut d = String::new();
    let mut e = String::new();

    println!("\nInput the First Diagonal of the Rhombus;");
    io::stdin().read_line(& mut d).expect("Failed to Read String");
    let diagonal2: f32 = d.trim().parse().expect("Invalid Input");

    println!("\nInput the Second Diagonal of the Rhombus;");
    io::stdin().read_line(& mut e).expect("Failed to Read String");
    let diagonal1: f32 = e.trim().parse().expect("Invalid String");

    let area = 1.0 / 2.0 * (diagonal1 * diagonal2);
    println!("\n--The Area of The Rhombus is {:.2}cm²--",area);
}
fn parallelogram() {
    let mut f = String::new();
    let mut g = String::new();

    println!("\nInput the Base of the Parallelogram;");
    io::stdin().read_line(& mut f).expect("Failed to Read String");
    let base: f32 = f.trim().parse().expect("Invalid Input");

    println!("\nInput the Altitude of the Parallelogram;");
    io::stdin().read_line(& mut g).expect("Failed to Read String");
    let altitude: f32 = g.trim().parse().expect("Invalid String");

    let area = base * altitude;
    println!("\n--The Area of The Parallelogram is {:.2}cm²--",area);
}
fn cube_a() {
    let mut h = String::new();
   
    println!("\nInput the Length of the Cube;");
    io::stdin().read_line(& mut h).expect("Failed to Read String");
    let length: f32 = h.trim().parse().expect("Invalid Input");

    let area = 6.0 * (length.powf(2.0));
    println!("\n--The Surface Area of The Cube is {:.2}cm²--",area);
}
fn cube_v() {
    let mut i = String::new();
   
    println!("\nInput the Length of the Cube;");
    io::stdin().read_line(& mut i).expect("Failed to Read String");
    let length: f32 = i.trim().parse().expect("Invalid Input");

    let volume = length.powf(3.0);
    println!("\n--The Volume of The Cube is {:.2}cm³--",volume);
}
fn cylinder() {
    let mut j = String::new();
    let mut k = String::new();

    println!("\nInput the The Radius of the Cylinder;");
    io::stdin().read_line(& mut j).expect("Failed to Read String");
    let radius: f32 = j.trim().parse().expect("Invalid Input");

    println!("\nInput the The Height of the Cylinder;");
    io::stdin().read_line(& mut k).expect("Failed to Read String");
    let height: f32 = k.trim().parse().expect("Invalid String");

    let volume = get_pi() * radius.powf(2.0) * height;
    println!("\n--The Volume of the Cylinder is {:.2}cm³--",volume);
}

