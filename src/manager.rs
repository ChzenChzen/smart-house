use crate::HouseError;

pub trait Manager {
    type Item;

    fn name(&self) -> &str;
    fn add(&mut self, item: Self::Item) -> Result<(), HouseError>;
    fn remove(&mut self, item: impl AsRef<str>) -> Result<(), HouseError>;
}
