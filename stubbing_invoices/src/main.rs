// The program must return all the issued invoices with values smaller than 100.
// The collection of invoices can be found in our database.
// The class IssuedInvoices already contains a method that retrieves all the invoices
pub struct InvoiceFilter {
  invoices_repo: Box<dyn InvoicesRepository>,
}

impl InvoiceFilter {
  pub fn new(invoices_repo: Box<dyn InvoicesRepository>) -> Self {
    Self { invoices_repo }
  }

  pub fn low_value_invoices(&self) -> Vec<Invoice> {
    let invoices = self.invoices_repo.all();

    invoices
      .into_iter()
      .filter(|invoice| invoice.value < 100)
      .collect()
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Invoice {
  to: String,
  value: i32,
}

#[cfg_attr(test, mockall::automock)]
pub trait InvoicesRepository {
  fn all(&self) -> Vec<Invoice>;
}

fn main() {
  println!("Hello, world!");
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn should_return_empty_list_when_there_are_no_invoices_to_filter() {
    let mut invoices_repo = MockInvoicesRepository::new();

    invoices_repo.expect_all().return_const(vec![]);

    let invoice_filter = InvoiceFilter::new(Box::new(invoices_repo));

    assert!(invoice_filter.low_value_invoices().is_empty());
  }

  #[test]
  fn single_low_value_invoice() {
    let mut invoices_repo = MockInvoicesRepository::new();

    let john = Invoice {
      to: String::from("John"),
      value: 99,
    };

    invoices_repo.expect_all().return_const(vec![john.clone()]);

    let invoice_filter = InvoiceFilter::new(Box::new(invoices_repo));

    assert_eq!(vec![john], invoice_filter.low_value_invoices());
  }

  #[test]
  fn single_high_value_invoice() {
    let mut invoices_repo = MockInvoicesRepository::new();

    let john = Invoice {
      to: String::from("John"),
      value: 100,
    };

    invoices_repo.expect_all().return_const(vec![john.clone()]);

    let invoice_filter = InvoiceFilter::new(Box::new(invoices_repo));

    assert!(invoice_filter.low_value_invoices().is_empty());
  }

  #[test]
  fn low_value_and_high_value_invoices() {
    let mut invoices_repo = MockInvoicesRepository::new();

    let mauricio = Invoice {
      to: String::from("Mauricio"),
      value: 20,
    };

    let steve = Invoice {
      to: String::from("Steve"),
      value: 99,
    };

    let frank = Invoice {
      to: String::from("Frank"),
      value: 100,
    };

    invoices_repo
      .expect_all()
      .return_const(vec![mauricio.clone(), steve.clone(), frank.clone()]);

    let invoice_filter = InvoiceFilter::new(Box::new(invoices_repo));

    let actual = invoice_filter.low_value_invoices();

    assert!(actual.contains(&mauricio));
    assert!(actual.contains(&steve));
  }
}
