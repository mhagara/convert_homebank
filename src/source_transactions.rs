use crate::homebank::HomeBankTransaction;

pub mod revolut;
pub mod unicredit;

pub trait MappableToHomeBank  {
    fn map_to_homebank(&self) -> HomeBankTransaction;
}

pub trait ReadFromPath {
    fn read_from_path(path: &str) -> Result<Self, &str>
    where
        Self: Sized;
}
