use crate::homebank::{HomeBankPaymentType, HomeBankTransaction};
use crate::source_transactions::{MappableToHomeBank, ReadFromPath};
use crate::utils::{to_decimal_direct, to_naive_datetime, to_string};
use chrono::NaiveDateTime;
use rust_decimal::Decimal;

pub type RevolutCollection = Vec<RevolutTransaction>;

pub enum RevolutTransactionType {
    None,
    Cashback,
    Transfer,
    CardPayment,
    Topup,
    CardRefund,
    Exchange,
    Atm,
}

pub struct RevolutTransaction {
    pub transaction_type: RevolutTransactionType,
    pub product: Option<String>,
    pub started_date: Option<NaiveDateTime>,
    pub completed_date: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub amount: Option<Decimal>,
    pub fee: Option<Decimal>,
    pub currency: Option<String>,
    pub state: Option<String>,
    pub balance: Option<Decimal>,
}

impl MappableToHomeBank for RevolutTransaction {
    fn map_to_homebank(&self) -> HomeBankTransaction {
        return HomeBankTransaction {
            date: match self.started_date {
                Some(d) => Some(d.date()),
                _ => None
            },
            payment: match self.transaction_type {
                RevolutTransactionType::Cashback => HomeBankPaymentType::BankTransfer,
                RevolutTransactionType::Transfer => HomeBankPaymentType::BankTransfer,
                RevolutTransactionType::CardPayment => HomeBankPaymentType::DebitCard,
                RevolutTransactionType::Topup => HomeBankPaymentType::BankTransfer,
                RevolutTransactionType::CardRefund => HomeBankPaymentType::DebitCard,
                RevolutTransactionType::Exchange => HomeBankPaymentType::BankTransfer,
                RevolutTransactionType::Atm => HomeBankPaymentType::Cash,
                _ => HomeBankPaymentType::None,
            },
            info: None,
            payee: None,
            memo: self.description.clone(),
            amount: self.amount.clone(),
            category: None,
            tags: None,
        };
    }
}

impl ReadFromPath for RevolutCollection {
    fn read_from_path(path: &str) -> Result<Self, &str> {
        if let Ok(mut reader) = csv::Reader::from_path(path) {
            return Ok(reader
                .records()
                .filter_map(|record_result| {
                    if let Ok(record) = record_result {
                        return Some(RevolutTransaction {
                            transaction_type: match record.get(0) {
                                Some(transaction_type) => {
                                    match transaction_type.to_uppercase().as_str() {
                                        "CASHBACK" => RevolutTransactionType::Cashback,
                                        "TRANSFER" => RevolutTransactionType::Transfer,
                                        "CARD_PAYMENT" => RevolutTransactionType::CardPayment,
                                        "TOPUP" => RevolutTransactionType::Topup,
                                        "CARD_REFUND" => RevolutTransactionType::CardRefund,
                                        "EXCHANGE" => RevolutTransactionType::Exchange,
                                        "ATM" => RevolutTransactionType::Atm,
                                        _ => RevolutTransactionType::None,
                                    }
                                }
                                _ => RevolutTransactionType::None,
                            },
                            product: to_string(record.get(1)),
                            started_date: to_naive_datetime(record.get(2), "%Y-%m-%d %H:%M:%S"),
                            completed_date: to_naive_datetime(record.get(3), "%Y-%m-%d %H:%M:%S"),
                            description: to_string(record.get(4)),
                            amount: to_decimal_direct(record.get(5)),
                            fee: to_decimal_direct(record.get(6)),
                            currency: to_string(record.get(7)),
                            state: to_string(record.get(8)),
                            balance: to_decimal_direct(record.get(9)),
                        });
                    }
                    return None;
                })
                .collect());
        }
        return Err("");
    }
}
