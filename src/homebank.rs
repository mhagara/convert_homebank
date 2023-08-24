use chrono::NaiveDate;
use rust_decimal::Decimal;
pub mod collection;
pub struct HomeBankTransaction {
    pub date: Option<NaiveDate>,
    pub payment: HomeBankPaymentType,
    pub info: Option<String>,
    pub payee: Option<String>,
    pub memo: Option<String>,
    pub amount: Option<Decimal>,
    pub category: Option<String>,
    pub tags: Option<String>,
}

#[derive(Copy, Clone)]
#[cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
pub enum HomeBankPaymentType {
    None = 0,
    CreditCard = 1,
    Check = 2,
    Cash = 3,
    BankTransfer = 4,
    // Not allowed because CSV do not support multiple accounts => will be imported as 4 = bank transfer.
    InternalTransfer = 5,
    DebitCard = 6,
    StandingOrder = 7,
    ElectronicPayment = 8,
    Deposit = 9,
    FinancialInstitutionFee = 10,
    DirectDebit = 11,
}
