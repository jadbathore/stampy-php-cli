use dialoguer::{Confirm, Input, MultiSelect, Select};
use phper::{
    arrays::ZArray, classes::{ClassEntity,Visibility}, values::ZVal
};
use console::Term;

pub fn class_dialoguer()->ClassEntity<()>
{ 
    let mut class:ClassEntity<()> = ClassEntity::new("Dialoguer");
    class.add_static_method("Confirm",Visibility::Public,method_confirm);
    class.add_static_method("input",Visibility::Public,method_input);
    class.add_static_method("select",Visibility::Public,method_select);
    class.add_static_method("MultiSelect",Visibility::Public,method_multi_select);
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
    let mut vec:Vec<&'a str> = Vec::new();
    let select_list = arguments[1].expect_z_arr()?.iter();
    for (_key,value) in select_list {
        let check_value:&'a str = value.expect_z_str()?.to_str()?;
        vec.push(check_value);
    }
    let term = Term::stdout();
    term.write_line("[Space] select | [Enter] valid | [↑↓] navigate | [a] select all").unwrap();
    let selection:Vec<usize> = MultiSelect::new().with_prompt(promps).items(&vec.clone()).interact_on(&term).unwrap();
    let mut arr:ZArray = ZArray::new();
    for i in selection.into_iter() {
        arr.insert(i as u64, ZVal::from(vec[i]));
    }
    term.flush().unwrap();
    Ok(arr)

}
