use crate::homebank::HomeBankTransaction;
use crate::utils::*;

pub type HomeBankTransactionCollection = Vec<HomeBankTransaction>;

pub trait WriteHomeBankCsv {
    fn write_csv(&self, path: &str) -> Result<(), Box<dyn std::error::Error>>;
}

impl WriteHomeBankCsv for HomeBankTransactionCollection {
    fn write_csv(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let writer_result = csv::WriterBuilder::new().delimiter(b';').from_path(path);
        if let Ok(mut writer) = writer_result {
            writer.write_record(&[
                "date", "payment", "info", "payee", "memo", "amount", "category", "tags",
            ])?;
            for t in self {
                writer.write_record(&[
                    naive_date_to_string(&t.date),
                    payment_to_string(&t.payment),
                    option_to_string(&t.info),
                    option_to_string(&t.payee),
                    option_to_string(&t.memo),
                    decimal_to_string(&t.amount),
                    option_to_string(&t.category),
                    option_to_string(&t.tags),
                ])?;
            }
            writer.flush()?;
        }
        return Ok(());
    }
}
