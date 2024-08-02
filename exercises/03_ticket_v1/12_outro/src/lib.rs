// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order {

    fn validate_product_name(value: &String) {
        if value.is_empty() {
            panic!()
        }
        if value.len() > 300 {
            panic!()
        }
    }
    fn validate_quantity(value: &u32) {
        if *value == 0 {
            panic!()
        }
    }
    fn validate_unit_price(value: &u32) {
        if *value == 0 {
            panic!()
        }
    }

    pub fn set_product_name(&mut self, value: String) {
        Order::validate_product_name(&value);
        self.product_name = value.to_string()
    }

    pub fn set_quantity(&mut self, value: u32) {
        Order::validate_quantity(&value);
        self.quantity = value
    }

    pub fn set_unit_price(&mut self, value: u32) {
        Order::validate_unit_price(&value);
        self.unit_price = value
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }
    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }
    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn total(&self) -> u32 {
        self.unit_price * self.quantity
    }

    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Order {
        Order::validate_product_name(&product_name);
        Order::validate_quantity(&quantity);
        Order::validate_unit_price(&unit_price);
        Order {
            product_name,
            quantity,
            unit_price,
        }
    }

}