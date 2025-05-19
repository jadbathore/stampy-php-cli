
use console::Term;
use dialoguer::{Confirm, Input, MultiSelect, Select,console};
use phper::{
    arrays::ZArray,
    classes::{ClassEntity,Visibility}, 
    values::ZVal
};
use crossterm::{ExecutableCommand};

use crate::enums::theme;
use crate::structs::handler;

pub fn class_dialoguer()->ClassEntity<()>
{ 
    let mut class:ClassEntity<()> = ClassEntity::new("Dialoguer");
    class.add_static_method("Confirm",Visibility::Public,method_confirm);
    class.add_static_method("input",Visibility::Public,method_input);
    class.add_static_method("select",Visibility::Public,method_select);
    class.add_static_method("MultiSelect",Visibility::Public,method_multi_select);
    // class.add_static_method("test",Visibility::Public,method_test);
    class
}

// in php = Dialoguer::confirm
fn method_confirm<'a>(arguments:&'a mut [ZVal])->Result<bool, phper::Error>
{
    let promps:&'a str = arguments[0].expect_z_str()?.to_str()?;
    let confirm = Confirm::new()
    .with_prompt(promps).interact().unwrap();
    Ok(confirm)
}

// in php = Dialoguer::input
fn method_input<'a>(arguments:&'a mut [ZVal])->Result<String, phper::Error>
{
    let promps:&'a str = arguments[0].expect_z_str()?.to_str()?;
    let input = Input::<String>::new()
    .with_prompt(promps).interact_text().unwrap();
    Ok(input)
}

// in php = Dialoguer::select
fn method_select<'a>(arguments:&'a mut [ZVal])->Result<String, phper::Error>
{
    let promps:&'a str = arguments[0].expect_z_str()?.to_str()?;
    let mut vec:Vec<String> = Vec::new();
    let select_list = arguments[1].expect_z_arr()?.iter();
    for (_key,value) in select_list {
        let check_value:String = value.expect_z_str()?.to_str()?.to_string();
        vec.push(check_value);
    }
    let selection = Select::new().with_prompt(promps).items(&vec).interact().unwrap();
    Ok(vec[selection].clone())
}

// in php = Dialoguer::multiSelect
fn method_multi_select<'a>(arguments:&'a mut [ZVal])->Result<ZArray, phper::Error>
{
    let promps:&'a str = arguments[0].expect_z_str()?.to_str()?;
    let vec:Vec<&'a str> = list_maker(&arguments[1])?;
    let mut stdout:Term = Term::buffered_stdout();
    stdout.execute(crossterm::cursor::MoveTo(5,5)).unwrap();
    stdout.write_line("[Space] select | [Enter] valid | [↑↓] navigate | [a] select all").unwrap();
    let selection:Vec<usize> = MultiSelect::new().with_prompt(promps).items(&vec.clone()).interact_on(&stdout).unwrap();
    let arr:ZArray = list_handler(selection, vec);
    Ok(arr)
}

fn list_maker<'a>(z_value:&'a ZVal)->Result<Vec<&'a str>, phper::Error>{
    let mut vec:Vec<&'a str> = Vec::new();
    let select_list = z_value.expect_z_arr()?.iter();
    for (_key,value) in select_list {
        let check_value:&'a str = value.expect_z_str()?.to_str()?;
        vec.push(check_value);
    }
    Ok(vec)
}

fn list_handler<'a>(selection:Vec<usize>,values:Vec<&'a str>)->ZArray
{
    let mut arr:ZArray = ZArray::new();
    for i in selection.into_iter() {
        arr.insert(i as u64, ZVal::from(values[i]));
    }
    arr
}




fn method_test<'a>(arguments:&'a mut [ZVal])->Result<ZArray, phper::Error>
{
    let arg_list = arguments.iter();
    if let (Some(promps),Some(list),Some(theme),None) = (arg_list.next(),arg_list.next(),arg_list.next(),arg_list.next()){
        let promps:&'a str = promps.expect_z_str()?.to_str()?;
        let list:Vec<&'a str> = list_maker(list)?;
        let mut stdout:Term = Term::buffered_stdout();
        stdout.execute(crossterm::cursor::MoveTo(5,5)).unwrap();
        stdout.write_line("[Space] select | [Enter] valid | [↑↓] navigate | [a] select all").unwrap();
        let theme = theme.expect_mut_z_obj()?.call(&"value",[]).unwrap();
        let selection:Vec<usize> = MultiSelect::with_theme(theme).with_prompt(promps).items(&list.clone()).interact_on(&stdout).unwrap();
        let arr:ZArray = list_handler(selection, list);
        Ok(list_handler(selection, list))
    } else {
        method_multi_select(arguments)
    }
}

