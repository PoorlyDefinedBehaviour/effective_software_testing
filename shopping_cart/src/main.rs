use typed_builder::TypedBuilder;

#[derive(Debug)]
pub struct ShoppingCart {
  items: Vec<CartItem>,
}

impl ShoppingCart {
  pub fn new() -> Self {
    Self { items: vec![] }
  }

  pub fn add(&mut self, item: CartItem) {
    self.items.push(item);
  }

  pub fn total_price_in_cents(&self) -> u64 {
    self
      .items
      .iter()
      .map(|item| item.price_in_cents * item.quantity)
      .sum::<u64>()
  }
}

#[derive(Debug, TypedBuilder)]
pub struct CartItem {
  product_id: u64,
  price_in_cents: u64,
  quantity: u64,
}

fn main() {
  println!("Hello, world!");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn empty_cart() {
    let cart = ShoppingCart::new();
    assert_eq!(0, cart.total_price_in_cents());
  }

  #[test]
  fn cart_with_one_item_quantity_1() {
    let mut cart = ShoppingCart::new();
    cart.add(
      CartItem::builder()
        .product_id(1)
        .price_in_cents(1000)
        .quantity(1)
        .build(),
    );
    assert_eq!(1000, cart.total_price_in_cents());
  }

  #[test]
  fn cart_with_with_one_item_and_quantity_greater_than_1() {
    let mut cart = ShoppingCart::new();
    cart.add(
      CartItem::builder()
        .product_id(1)
        .price_in_cents(1000)
        .quantity(2)
        .build(),
    );
    assert_eq!(2000, cart.total_price_in_cents());
  }

  #[test]
  fn cart_with_many_items_quantity_one() {
    let mut cart = ShoppingCart::new();
    cart.add(
      CartItem::builder()
        .product_id(1)
        .price_in_cents(1000)
        .quantity(1)
        .build(),
    );
    cart.add(
      CartItem::builder()
        .product_id(2)
        .price_in_cents(1000)
        .quantity(1)
        .build(),
    );
    assert_eq!(2000, cart.total_price_in_cents());
  }

  #[test]
  fn cart_with_many_items_quantity_greater_than_1() {
    let mut cart = ShoppingCart::new();
    cart.add(
      CartItem::builder()
        .product_id(1)
        .price_in_cents(1000)
        .quantity(3)
        .build(),
    );
    cart.add(
      CartItem::builder()
        .product_id(2)
        .price_in_cents(1000)
        .quantity(2)
        .build(),
    );
    assert_eq!(5000, cart.total_price_in_cents());
  }
}
