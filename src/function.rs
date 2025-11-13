use phper::{errors::ArgumentCountError, values::ZVal};
use dialoguer::console::Style;

use crate::paddingPrintln;

pub fn padding(arguments:&mut [ZVal])->Result<(),phper::Error>{
    let mut arg_list = arguments.iter();
    let arguments_expected  = (arg_list.next(),arg_list.next());
    if let (Some(input),None) = arguments_expected {
        let str_input = input.expect_z_str()?.to_str()?;        
        paddingPrintln!(str_input);
        Ok(())
    } else {
        Err(phper::Error::ArgumentCount(ArgumentCountError::new(String::from("push"), 1, arguments.iter().len())))
    }   
}