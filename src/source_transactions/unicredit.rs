use crate::homebank::{HomeBankPaymentType, HomeBankTransaction};
use crate::source_transactions::{MappableToHomeBank, ReadFromPath};
use crate::utils::{to_decimal, to_naive_date, to_string};
use calamine::{Reader, Xlsx};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use std::fs::File;
use std::io::BufReader;

pub type UniCreditCollection = Vec<UniCreditTransaction>;

pub struct UniCreditTransaction {
    pub amount: Option<Decimal>,
    pub status: Option<String>,
    pub date: Option<NaiveDate>,
    pub partner_account: Option<String>,
    pub partner: Option<String>,
    pub details: Option<String>,
    pub transaction_type: Option<String>,
}

impl MappableToHomeBank for UniCreditTransaction {
    fn map_to_homebank(&self) -> HomeBankTransaction {
        return HomeBankTransaction {
            amount: self.amount.clone(),
            category: None,
            date: self.date,
            info: None,
            memo: self.details.clone(),
            payee: self.partner.clone(),
            payment: match (self.transaction_type.clone(), self.details.clone()) {
                (Some(t), _) if t.as_str() == "Bejövő Forint Átutalás" => {
                    HomeBankPaymentType::BankTransfer
                }
                (Some(t), _) if t.as_str() == "Bejövő azonnali megbízás" => {
                    HomeBankPaymentType::BankTransfer
                }
                (Some(t), _) if t.as_str() == "Kártyatranzakció" => {
                    HomeBankPaymentType::DebitCard
                }
                (_, _) => HomeBankPaymentType::None,
            },
            tags: None,
        };
    }
}

impl ReadFromPath for UniCreditCollection {
    fn read_from_path(path: &str) -> Result<Self, &str> {
        let mut workbook: Xlsx<BufReader<File>> = match calamine::open_workbook(path) {
            Ok(w) => w,
            Err(_) => return Err(""),
        };

        let range = match workbook.sheet_names().first().cloned() {
            Some(first_sheet_name) => match workbook.worksheet_range(first_sheet_name.as_str()) {
                Some(Ok(range)) => range,
                Some(Err(_)) => return Err(""),
                None => return Err(""),
            },
            None => return Err(""),
        };

        let mut result = Vec::new();

        let mut row_index = 0;
        let mut column_index;
        for row in range.rows() {
            column_index = 0;
            if row_index > 3 {
                let mut item =UniCreditTransaction {
                    amount: None,
                    status: None,
                    date: None,
                    partner_account: None,
                    partner: None,
                    details: None,
                    transaction_type: None,
                };
                for cell in row.iter() {
                    if column_index == 1 {
                        item.amount = to_decimal(cell.get_string());
                    } else if column_index == 3 {
                        item.status = to_string(cell.get_string());
                    } else if column_index == 4 {
                        item.date = to_naive_date(cell.get_string(), "%Y.%m.%d");
                    } else if column_index == 5 {
                        item.partner_account = to_string(cell.get_string());
                    } else if column_index == 6 {
                        item.partner = to_string(cell.get_string());
                    } else if column_index == 8 {
                        item.details = to_string(cell.get_string());
                    } else if column_index == 9 {
                        item.transaction_type = to_string(cell.get_string());
                    }
                    column_index += 1;
                }
                result.push(item);
            }
            row_index += 1;
        }

        return Ok(result);
    }
}
