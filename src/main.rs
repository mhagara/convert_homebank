use crate::homebank::collection::{HomeBankTransactionCollection, WriteHomeBankCsv};
use crate::source_transactions::revolut::{RevolutCollection, RevolutTransaction};
use crate::source_transactions::unicredit::{UniCreditCollection, UniCreditTransaction};
use crate::source_transactions::MappableToHomeBank;
use crate::source_transactions::ReadFromPath;
use crate::utils::{option_equals_to_str_ignore_case, to_naive_date};
use clap::{Parser, ValueEnum};

mod homebank;
mod source_transactions;
mod utils;

/// Converts Revolut or UniCredit statements to HomeBank CSV format.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Source from Revolut CSV or UniCredit XLS.
    #[arg(long, short)]
    input: String,
    /// Destination HomeBank CSV.
    #[arg(long, short)]
    output: String,
    /// Conversion type Revolut or UniCredit.
    #[arg(long, short)]
    conversion: ConversionType,
    /// Filter transactions from date (inclusive). (yyyy-mm-dd)
    #[arg(long, short)]
    from: Option<String>,
    /// Filter transactions to date (inclusive). (yyyy-mm-dd)
    #[arg(long, short)]
    to: Option<String>,
    /// Include all transactions, not just completed or booked
    #[arg(short = 'a', long, default_value_t = false)]
    include_all: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[clap(rename_all = "verbatim")]
enum ConversionType {
    /// Convert from Revolut CSV
    Revolut,
    /// Convert from UniCredit XLS
    UniCredit,
}

fn main() {
    let args = Args::parse();

    let mut transactions: Vec<Box<dyn MappableToHomeBank>> = Vec::new();

    if args.conversion == ConversionType::Revolut {
        let filters: [Box<fn(&RevolutTransaction, args: &Args) -> bool>; 2] = [
            Box::new(|i, _| option_equals_to_str_ignore_case(&i.product, "Current")),
            Box::new(|i, args| { return args.include_all || option_equals_to_str_ignore_case(&i.state, "COMPLETED"); }),
        ];

        for t in RevolutCollection::read_from_path(args.input.as_str())
            .expect("Failed to read Revolut CSV")
        {
            if filters.iter().all(|predicate| predicate(&t, &args)) {
                transactions.push(Box::new(t));
            }
        }
    } else if args.conversion == ConversionType::UniCredit {
        let filters: [Box<fn(&UniCreditTransaction, args: &Args) -> bool>; 1] =
            [Box::new(|i, args| {
                return args.include_all || option_equals_to_str_ignore_case(&i.status, "Könyvelt");
            })];
        for t in UniCreditCollection::read_from_path(args.input.as_str())
            .expect("Failed to read UniCredit CSV")
        {
            if filters.iter().all(|predicate| predicate(&t, &args)) {
                transactions.push(Box::new(t));
            }
        }
    }

    let mut homebank_transactions: HomeBankTransactionCollection =
        transactions.iter().map(|i| i.map_to_homebank()).collect();

    if let Some(from_date) = to_naive_date(args.from.as_deref(), "%Y-%m-%d") {
        homebank_transactions.retain(|i| {
            if let Some(d) = i.date {
                if d >= from_date {
                    return true;
                }
            }
            return false;
        });
    }

    if let Some(to_date) = to_naive_date(args.to.as_deref(), "%Y-%m-%d") {
        homebank_transactions.retain(|i| {
            if let Some(d) = i.date {
                if d <= to_date {
                    return true;
                }
            }
            return false;
        });
    }

    homebank_transactions
        .write_csv(args.output.as_str())
        .expect("Failed to write HomeBank CSV");
}
