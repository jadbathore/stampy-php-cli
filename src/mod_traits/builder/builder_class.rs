
use phper::{
    values::ZVal,
    arrays::ZArray,
    functions::MethodEntity
};

use dialoguer::MultiSelect;


pub trait BuilderClass<'a> {
    type OutputType;
    fn set_class(&mut self);
    fn set_input_and_optionnal_theme_args(&mut self,method_entity:&mut MethodEntity);
    fn set_input_list_and_optionnal_theme_args(&mut self,method_entity:&mut MethodEntity);
    fn confirm(arguments:&mut [ZVal])->Result<bool, phper::Error>;
    fn build(&mut self) -> Self::OutputType;
    fn input(arguments:&'a mut [ZVal])->Result<String, phper::Error>;
    fn select(arguments:&'a mut [ZVal])->Result<String, phper::Error>;
    fn multi_select(arguments:&'a mut [ZVal])->Result<ZArray, phper::Error>;
    fn password(arguments:&'a mut [ZVal])-> Result<(),phper::Error>;
    fn method_mutli_select_handler(arguments:&'a mut [ZVal],multi_select_instance:MultiSelect)->Result<ZArray, phper::Error>;
    fn list_maker(z_value:&'a ZVal)->Result<Vec<&'a str>, phper::Error>;
    fn list_handler(selection:Vec<usize>,values:Vec<&'a str>)->ZArray;
}

