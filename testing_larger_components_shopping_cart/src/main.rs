//! Given a shopping cart with items, quantities and respective unit prices, the
//! final price of the cart is calculated as follows:
//! The final price of each item is calculated by multiplying its unit price by
//! the quantity
//! The delivery costs are the following. For shopping carts with:
//! 1 up to 3 elements (inclusive), we charge 5 euros extra.
//! 4 up to 10 elements (inclusive), we charge 12.5 euros extra.
//! more than 10 elements, we charge 20 euros extra.
//! If there is any electronic item in the cart, we charge 7.50 euros extra.

use std::str::FromStr;

use bigdecimal::BigDecimal;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq)]
pub struct ShoppingCart {
  items: Vec<Item>,
}

impl ShoppingCart {
  pub fn new() -> Self {
    Self { items: vec![] }
  }

  pub fn number_of_items(&self) -> usize {
    self.items.len()
  }

  pub fn add(&mut self, item: Item) {
    self.items.push(item);
  }

  pub fn items(&self) -> &Vec<Item> {
    &self.items
  }
}

#[derive(Debug, Clone, PartialEq, TypedBuilder)]
pub struct Item {
  name: String,
  quantity: usize,
  price: BigDecimal,
  category: Category,
}

impl Item {
  pub fn category(&self) -> Category {
    self.category.clone()
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Category {
  Normal,
  Eletronic,
}

#[cfg_attr(test, mockall::automock)]
pub trait PriceRule {
  fn price_to_aggregate(&self, cart: &ShoppingCart) -> BigDecimal;
}

pub struct DeliveryPrice;

impl PriceRule for DeliveryPrice {
  fn price_to_aggregate(&self, cart: &ShoppingCart) -> BigDecimal {
    let out = match cart.number_of_items() {
      0 => "0.0",
      1..=3 => "5.0",
      4..=10 => "12.5",
      _ => "20.0",
    };

    BigDecimal::from_str(out).unwrap()
  }
}

pub struct ExtraChargeForElectronics;

impl PriceRule for ExtraChargeForElectronics {
  fn price_to_aggregate(&self, cart: &ShoppingCart) -> BigDecimal {
    let items = cart.items();

    if items
      .iter()
      .any(|item| item.category() == Category::Eletronic)
    {
      return BigDecimal::from_str("7.50").unwrap();
    }

    BigDecimal::from_str("0.0").unwrap()
  }
}

#[derive(TypedBuilder)]
pub struct PriceCalculator {
  rules: Vec<Box<dyn PriceRule>>,
}

impl PriceCalculator {
  pub fn calculate(&self, cart: &ShoppingCart) -> BigDecimal {
    self
      .rules
      .iter()
      .map(|rule| rule.price_to_aggregate(&cart))
      .sum()
  }
}

fn main() {
  println!("Hello, world!");
}

#[cfg(test)]
mod delivery_price_tests {
  use super::*;

  fn item() -> Item {
    Item::builder()
      .name(String::from("Name"))
      .quantity(1)
      .price(BigDecimal::from_str("10.0").unwrap())
      .category(Category::Normal)
      .build()
  }

  #[test]
  fn price_depends_on_the_number_of_items() {
    let tests = vec![
      (0, "0.0"),
      (1, "5.0"),
      (3, "5.0"),
      (4, "12.5"),
      (10, "12.5"),
      (11, "20.0"),
    ];

    for (num_items, expected) in tests {
      let mut cart = ShoppingCart::new();

      for _ in 0..num_items {
        cart.add(item());
      }

      assert_eq!(
        BigDecimal::from_str(expected).unwrap(),
        DeliveryPrice.price_to_aggregate(&cart)
      );
    }
  }
}

#[cfg(test)]
mod extra_charge_for_eletronics_tests {
  use super::*;

  #[test]
  fn empty_cart() {
    let cart = ShoppingCart::new();
    assert_eq!(
      BigDecimal::from_str("0.0").unwrap(),
      ExtraChargeForElectronics.price_to_aggregate(&cart)
    );
  }

  #[test]
  fn cart_does_not_contain_eletronic_item() {
    let mut cart = ShoppingCart::new();

    cart.add(
      Item::builder()
        .name(String::from("Name"))
        .quantity(1)
        .price(BigDecimal::from_str("10.0").unwrap())
        .category(Category::Normal)
        .build(),
    );

    assert_eq!(
      BigDecimal::from_str("0.0").unwrap(),
      ExtraChargeForElectronics.price_to_aggregate(&cart)
    );
  }

  #[test]
  fn cart_contains_electronic_item() {
    let mut cart = ShoppingCart::new();
    cart.add(
      Item::builder()
        .name(String::from("Name"))
        .quantity(1)
        .price(BigDecimal::from_str("10.0").unwrap())
        .category(Category::Normal)
        .build(),
    );
    cart.add(
      Item::builder()
        .name(String::from("Name"))
        .quantity(1)
        .price(BigDecimal::from_str("10.0").unwrap())
        .category(Category::Eletronic)
        .build(),
    );
    assert_eq!(
      BigDecimal::from_str("7.50").unwrap(),
      ExtraChargeForElectronics.price_to_aggregate(&cart)
    );
  }
}

#[cfg(test)]
mod price_calculator_tests {
  use super::*;
  use mockall::predicate::*;

  #[test]
  fn call_price_rules() {
    let mut rule_1 = MockPriceRule::new();
    let mut rule_2 = MockPriceRule::new();
    let mut rule_3 = MockPriceRule::new();

    let mut cart = ShoppingCart::new();

    cart.add(
      Item::builder()
        .name(String::from("Name"))
        .quantity(1)
        .price(BigDecimal::from_str("10.0").unwrap())
        .category(Category::Normal)
        .build(),
    );

    rule_1
      .expect_price_to_aggregate()
      .with(eq(cart.clone()))
      .return_const(BigDecimal::from_str("1.0").unwrap());

    rule_2
      .expect_price_to_aggregate()
      .with(eq(cart.clone()))
      .return_const(BigDecimal::from_str("0.0").unwrap());

    rule_3
      .expect_price_to_aggregate()
      .with(eq(cart.clone()))
      .return_const(BigDecimal::from_str("2.0").unwrap());

    let sut = PriceCalculator::builder()
      .rules(vec![Box::new(rule_1), Box::new(rule_2), Box::new(rule_3)])
      .build();

    assert_eq!(BigDecimal::from_str("3.0").unwrap(), sut.calculate(&cart));
  }
}
