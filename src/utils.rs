use crate::homebank::*;
use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::Decimal;
use std::str::FromStr;

pub fn to_naive_datetime(s: Option<&str>, fmt: &str) -> Option<NaiveDateTime> {
    return match s {
        Some(s) => match NaiveDateTime::parse_from_str(s, fmt) {
            Ok(d) => Some(d),
            Err(_) => None,
        },
        None => None,
    };
}

pub fn to_naive_date(s: Option<&str>, fmt: &str) -> Option<NaiveDate> {
    return match s {
        Some(s) => match NaiveDate::parse_from_str(s, fmt) {
            Ok(d) => Some(d),
            Err(_) => None,
        },
        None => None,
    };
}

pub fn to_string(s: Option<&str>) -> Option<String> {
    return match s {
        Some(s) => Some(s.to_string()),
        None => None,
    };
}

pub fn to_decimal_direct(s: Option<&str>) -> Option<Decimal> {
    return match s {
        Some(s) => {
            let d = Decimal::from_str(s);
            match d {
                Ok(d) => Some(d),
                Err(_) => None,
            }
        }
        None => None,
    };
}

pub fn to_decimal(s: Option<&str>) -> Option<Decimal> {
    return match s {
        Some(s) => {
            let clean: String = s
                .chars()
                .filter_map(|c| {
                    if c.is_ascii_digit() {
                        return Some(c);
                    }
                    if c == '-' {
                        return Some(c);
                    }
                    if c == ',' {
                        return Some('.');
                    }
                    return None;
                })
                .collect();
            let d = Decimal::from_str(clean.as_str());
            match d {
                Ok(d) => Some(d),
                Err(_) => None,
            }
        }
        None => None,
    };
}

pub fn naive_date_to_string(value: &Option<NaiveDate>) -> String {
    if let Some(d) = value {
        return d.format("%Y-%m-%d").to_string();
    }
    return String::new();
}

pub fn option_to_string(value: &Option<String>) -> String {
    if let Some(s) = value {
        return s.clone();
    }
    return String::new();
}

pub fn decimal_to_string(value: &Option<Decimal>) -> String {
    if let Some(d) = value {
        return d.to_string();
    }
    return String::new();
}

pub fn payment_to_string(value: &HomeBankPaymentType) -> String {
    return (*value as u8).to_string();
}

pub fn option_equals_to_str_ignore_case(a: &Option<String>, b: &str) -> bool {
    if let Some(s) = a {
        return s.to_lowercase() == b.to_lowercase();
    }
    return false;
}
