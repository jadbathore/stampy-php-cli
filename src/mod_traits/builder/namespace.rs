
pub trait BuilderNamespaceBuf {
    type OutputType;
    fn set_items(&mut self,namespace:&str);
    fn set_inner(&mut self);
    fn build(self) -> Self::OutputType;
}