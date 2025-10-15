<<<<<<< HEAD
fn main () {
	let p:f64 = 520_000_000.0;
	let r:f64 = 10.0;
	let n = 5.0;

// Compound Interest
	let a = p * (1.0 + (r / 100.0)).powf(n);
	println!("Amount is  ₦{:.2}", a );
	let ci = a - p;
	println!("Compound Interest is  ₦{:.2}", ci);

=======
fn main () {
	let p:f64 = 520_000_000.0;
	let r:f64 = 10.0;
	let n = 5.0;

// Compound Interest
	let a = p * (1.0 + (r / 100.0)).powf(n);
	println!("Amount is  ₦{:.2}", a );
	let ci = a - p;
	println!("Compound Interest is  ₦{:.2}", ci);

>>>>>>> bcb932e7cec77ad48a300c0f084ee2d1ac432e19
}