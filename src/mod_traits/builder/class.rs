pub trait BuilderClass
{
    const CLASS_NAME:&str;
    type OutputType;
    fn set_class(&mut self);
    fn set_methods(&mut self);
    fn build(self) -> Self::OutputType;
}


// pub trait BuilderBoundClass {
//     type OutputType;
//     fn set_class(&mut self,class_name:&str);
//     fn set_methods(&mut self);
//     fn build(self) -> Self::OutputType;
// }


pub trait BuilderEnum {
    type OutputType;
    fn set_enum(&mut self,class_name:&str);
    fn set_case(&mut self);
    fn build(self) -> Self::OutputType;
}

pub trait BuilderPropertyClass {
    type OutputType;
    fn set_class(&mut self,class_name:&str);
    fn set_methods(&mut self);
    fn set_property(&mut self);
    fn build(self) -> Self::OutputType;
}

pub trait BuilderWrapper {
    type OutputType;
    type InputType;
    fn set_item(&mut self);
    fn build(self) -> Self::OutputType;
}