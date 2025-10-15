<<<<<<< HEAD
fn main () {
	let toshiba:f64 = 450_000.00;
	let mac:f64 = 1_500_000.00;
	let hp:f64 = 750_000.00;
	let dell:f64 = 2_850_000.00;
	let acer:f64 = 250_000.00;
	let t_q: f64 = 2.0;
	let m_q: f64 = 1.0;
	let h_q: f64 = 3.0;
	let d_q: f64 = 3.0;
	let a_q: f64 = 1.0;

	let t_sum = toshiba * t_q;
	let m_sum = mac * m_q;
	let h_sum = hp * h_q;
	let d_sum = dell * d_q;
	let a_sum = acer * a_q;

	let sum = t_sum + m_sum + h_sum + d_sum + a_sum;
	println!("The Sum is ₦{:.2}", sum);

	let total_qty = t_q + m_q + h_q + d_q + a_q;
	let average: f64 = sum / total_qty;
	
	println!("The Average is ₦{:.2}", average);
=======
fn main () {
	let toshiba:f64 = 450_000.00;
	let mac:f64 = 1_500_000.00;
	let hp:f64 = 750_000.00;
	let dell:f64 = 2_850_000.00;
	let acer:f64 = 250_000.00;
	let t_q: f64 = 2.0;
	let m_q: f64 = 1.0;
	let h_q: f64 = 3.0;
	let d_q: f64 = 3.0;
	let a_q: f64 = 1.0;

	let t_sum = toshiba * t_q;
	let m_sum = mac * m_q;
	let h_sum = hp * h_q;
	let d_sum = dell * d_q;
	let a_sum = acer * a_q;

	let sum = t_sum + m_sum + h_sum + d_sum + a_sum;
	println!("The Sum is ₦{:.2}", sum);

	let total_qty = t_q + m_q + h_q + d_q + a_q;
	let average: f64 = sum / total_qty;
	
	println!("The Average is ₦{:.2}", average);
>>>>>>> bcb932e7cec77ad48a300c0f084ee2d1ac432e19
}