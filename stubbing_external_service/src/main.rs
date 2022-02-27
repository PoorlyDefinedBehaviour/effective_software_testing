mod invoice_filter {
  #[derive(Debug, Clone, PartialEq)]
  pub struct Invoice {
    pub to: String,
    pub value: i32,
  }

  #[cfg_attr(test, mockall::automock)]
  pub trait InvoicesRepository {
    fn all(&self) -> Vec<Invoice>;
  }

  pub struct InvoiceFilter {
    invoices_repo: Box<dyn InvoicesRepository>,
  }

  #[cfg_attr(test, mockall::automock)]
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
}

use invoice_filter::Invoice;
#[mockall_double::double]
use invoice_filter::InvoiceFilter;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct SapInvoiceSender {
  invoice_filter: InvoiceFilter,
  sap: Box<dyn Sap>,
}

impl SapInvoiceSender {
  pub fn send_low_valued_invoices(&self) {
    for invoice in self.invoice_filter.low_value_invoices() {
      self.sap.send(&invoice);
    }
  }
}

#[cfg_attr(test, mockall::automock)]
pub trait Sap {
  fn send(&self, invoice: &Invoice);
}

fn main() {
  println!("Hello, world!");
}

#[cfg(test)]
mod tests {
  use super::*;
  use invoice_filter::MockInvoicesRepository;
  use mockall::predicate::*;

  #[test]
  fn sends_low_valued_invoices_to_sap() {
    // Note that we are mocking the InvoiceFilter itself instead of mocking
    // the repository used by it.
    let mut invoice_filter = InvoiceFilter::default();

    let mauricio = Invoice {
      to: String::from("Mauricio"),
      value: 20,
    };

    let steve = Invoice {
      to: String::from("Steve"),
      value: 99,
    };

    invoice_filter
      .expect_low_value_invoices()
      .return_const(vec![mauricio.clone(), steve.clone()]);

    let mut sap = MockSap::new();

    sap.expect_send().with(eq(mauricio)).return_const(());
    sap.expect_send().with(eq(steve)).return_const(());

    let sender = SapInvoiceSender::builder()
      .invoice_filter(invoice_filter)
      .sap(Box::new(sap))
      .build();

    sender.send_low_valued_invoices();
  }

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
