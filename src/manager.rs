pub trait Manager {
    type Output;

    fn name(&self) -> &str;
    fn list(&self) -> &[Self::Output];
    fn add(&mut self, item: Self::Output);
    fn remove(&mut self, item: &str);
}
