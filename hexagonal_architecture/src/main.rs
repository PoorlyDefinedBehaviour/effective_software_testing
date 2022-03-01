//! For all the shopping carts that were paid today, the system should:
//! Set the status of the shopping cart as "ready for delivery" and persist its
//! new state in the database
//! Notify the delivery center and let them know they should start working on
//! sending the goods to the customer
//! Notify the SAP system
//! Send an e-mail to the customer, confirming that the payment was
//! successful. The e-mail should contain an estimate of when the delivery
//! will happen. The information is available via the delivery center API.

mod repositories {
  use crate::ShoppingCart;

  pub struct ShoppinCartRepository;

  #[cfg_attr(test, mockall::automock)]
  impl ShoppinCartRepository {
    pub fn get_carts_paid_today(&self) -> Vec<ShoppingCart> {
      todo!()
    }

    pub fn save(&self, _cart: &ShoppingCart) {}
  }
}

mod delivery {
  use crate::ShoppingCart;

  pub struct DeliveryCenter;

  #[cfg_attr(test, mockall::automock)]
  impl DeliveryCenter {
    pub fn deliver(&self, _cart: &ShoppingCart) -> u8 {
      1
    }
  }
}

mod sap {
  use crate::ShoppingCart;

  pub struct Sap;

  #[cfg_attr(test, mockall::automock)]
  impl Sap {
    pub fn cart_ready_for_delivery(&self, _cart: &ShoppingCart) {}
  }
}

mod notifications {
  use crate::ShoppingCart;

  pub struct Notifier;

  #[cfg_attr(test, mockall::automock)]
  impl Notifier {
    pub fn send_estimated_delivery_notification(&self, _cart: &ShoppingCart) {}
  }
}

#[mockall_double::double]
use delivery::DeliveryCenter;
#[mockall_double::double]
use notifications::Notifier;
#[mockall_double::double]
use repositories::ShoppinCartRepository;
#[mockall_double::double]
use sap::Sap;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct PaidShoppingCartsBatch {
  shopping_cart_repo: ShoppinCartRepository,
  delivery_center: DeliveryCenter,
  sap: Sap,
  notifier: Notifier,
}

impl PaidShoppingCartsBatch {
  pub fn process_all(&self) {
    for mut cart in self.shopping_cart_repo.get_carts_paid_today() {
      let estimated_delivery_day = self.delivery_center.deliver(&cart);

      cart.mask_as_ready_for_delivery(estimated_delivery_day);

      // NOTE: The way this code is written is a problem if an error happens.
      // We should send a notification to SNS and have two lambdas
      // consume the SNS and then persist the updated cart in the database.
      self.shopping_cart_repo.save(&cart);

      self.notifier.send_estimated_delivery_notification(&cart);

      self.sap.cart_ready_for_delivery(&cart);
    }
  }
}

pub struct ShoppingCart {}

impl ShoppingCart {
  pub fn mask_as_ready_for_delivery(&mut self, _estimated_delivery_day: u8) {}
}

fn main() {
  println!("Hello, world!");
}
