#[derive(Debug, Clone)]
pub struct ProductInventory {
    pub name: String,
    pub price: f64,
    pub quantity: i32,
}

impl ProductInventory {
    pub fn new(name: String, price: f64, quantity: i32) -> ProductInventory {
        ProductInventory {
            name,
            price,
            quantity,
        }
    }

    pub fn update_quantity(&mut self, quantity: i32) {
        self.quantity += quantity;
    }
}
