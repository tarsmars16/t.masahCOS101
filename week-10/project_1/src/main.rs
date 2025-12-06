struct Laptop {
    price:u32
}

impl Laptop {
    fn cost(&self, quantity:u32) -> u32 {
        self.price * quantity
    }

}
fn main() {
   let hp = Laptop {
        price:650_000,
   };
   let ibm = Laptop {
        price:755_000,
   };
   let toshiba = Laptop {
        price:550_000,
   };
   let dell = Laptop {
        price:850_000,
   };


   let total_cost = hp.cost(3) + ibm.cost(3) + toshiba.cost(3) + dell.cost(3);

   println!("The Total Cost of The Customer's Purchase: ₦{}",total_cost);
}
