
// use console::Term;
// use dialoguer::{console, theme::Theme, Confirm, Input, MultiSelect, Password, Select};
// use phper::{
//     arrays::ZArray, 
//     classes::{ClassEntity,Visibility}, 
//     functions::{Argument, MethodEntity}, 
//     values::ZVal,
//     types::{ArgumentTypeHint}
// };
// use crossterm::{ExecutableCommand};

// // use crate::enums::theme;



// pub fn class_dialoguer()->ClassEntity<()>
// { 
    // let mut class:ClassEntity<()> = ClassEntity::new("Dialoguer");
//     // class.add_static_property("password_state", Visibility::Private, password_state);
//     method_string_plus_theme(
//         class.add_static_method("Confirm",Visibility::Public,method_confirm)
//     );
//     method_string_plus_theme(
//         class.add_static_method("input",Visibility::Public,method_input)
//     );
    
//     class.add_static_method("input",Visibility::Public,method_input);
//     class.add_static_method("select",Visibility::Public,method_select);
//     class.add_static_method("MultiSelect",Visibility::Public,method_multi_select);
//     class.add_static_method("test",Visibility::Public,method_test);
//     class.add_static_method("password",Visibility::Public,method_password);
//     class
// }





// fn method_string_plus_theme<'a>(method_entity:&'a mut MethodEntity)
// {
//     method_entity
//     .argument(Argument::new("input").with_type_hint(ArgumentTypeHint::String))
//     .argument(Argument::new("theme")
//     .with_type_hint(ArgumentTypeHint::ClassEntry(String::from(r"\Theme")))
//     .optional());
// }

// // in php = Dialoguer::confirm
// fn method_confirm(arguments:&mut [ZVal])->Result<bool, phper::Error>
// {
//     let mut arg_list = arguments.iter();
//     let mut confirm:Confirm;
//     let promp:&str;
//     if let (Some(promps),Some(theme),None) = (arg_list.next(),arg_list.next(),arg_list.next()){
//         let value = theme.expect_mut_z_obj()?.get_property(&"value");
//         let Theme = DialoguerTheme::from_str(value.expect_z_str()?.to_str()?);
//         confirm = Confirm::with_theme(Theme.into())
//     } else {
//         confirm = Confirm::new()
//     }
//     let confirm = Confirm::new();
//     let value:&str = arguments[1].expect_mut_z_obj()?.get_property(&"value")?;
//     confirm.with_prompt(promps).interact().unwrap();
//     Ok(confirm)
// }

// fn match_value_theme(dialoguerTheme:DialoguerTheme)
// {
//     match dialoguerTheme {
//         DialoguerTheme::ColorfulTheme(theme)=>{
//             self::Default()
//         }
//     }
// }

// // in php = Dialoguer::input
// fn method_input<'a>(arguments:&'a mut [ZVal])->Result<String, phper::Error>
// {
//     let promps:&'a str = arguments[0].expect_z_str()?.to_str()?;
//     let input = Input::<String>::new()
//     .with_prompt(promps).interact_text().unwrap();
//     Ok(input)
// }

// // in php = Dialoguer::select
// fn method_select<'a>(arguments:&'a mut [ZVal])->Result<String, phper::Error>
// {
//     let promps:&'a str = arguments[0].expect_z_str()?.to_str()?;
//     let mut vec:Vec<String> = Vec::new();
//     let select_list = arguments[1].expect_z_arr()?.iter();
//     for (_key,value) in select_list {
//         let check_value:String = value.expect_z_str()?.to_str()?.to_string();
//         vec.push(check_value);
//     }
//     let selection = Select::new().with_prompt(promps).items(&vec).interact().unwrap();
//     Ok(vec[selection].clone())
// }

// // in php = Dialoguer::multiSelect
// fn method_multi_select<'a>(arguments:&'a mut [ZVal])->Result<ZArray, phper::Error>
// {
//     let promps:&'a str = arguments[0].expect_z_str()?.to_str()?;
//     let vec:Vec<&'a str> = list_maker(&arguments[1])?;
//     let mut stdout:Term = Term::buffered_stdout();
//     stdout.execute(crossterm::cursor::MoveTo(5,5)).unwrap();
//     stdout.write_line("[Space] select | [Enter] valid | [↑↓] navigate | [a] select all").unwrap();
//     let selection:Vec<usize> = MultiSelect::new().with_prompt(promps).items(&vec.clone()).interact_on(&stdout).unwrap();
//     let arr:ZArray = list_handler(selection, vec);
//     Ok(arr)

// }

// fn method_mutli_select_handler<'a>(arguments:&'a mut [ZVal],multi_select_instance:MultiSelect)->Result<ZArray, phper::Error>
// {
//     let promps:&'a str = arguments[0].expect_z_str()?.to_str()?;
//     let vec:Vec<&'a str> = list_maker(&arguments[1])?;
//     let mut stdout:Term = Term::buffered_stdout();
//     stdout.execute(crossterm::cursor::MoveTo(5,5)).unwrap();
//     stdout.write_line("[Space] select | [Enter] valid | [↑↓] navigate | [a] select all").unwrap();
//     let selection:Vec<usize> = multi_select_instance.with_prompt(promps).items(&vec.clone()).interact_on(&stdout).unwrap();
//     let arr:ZArray = list_handler(selection, vec);
//     Ok(arr)
// }

// fn list_maker<'a>(z_value:&'a ZVal)->Result<Vec<&'a str>, phper::Error>{
//     let mut vec:Vec<&'a str> = Vec::new();
//     let select_list = z_value.expect_z_arr()?.iter();
//     for (_key,value) in select_list {
//         let check_value:&'a str = value.expect_z_str()?.to_str()?;
//         vec.push(check_value);
//     }
//     Ok(vec)
// }

// fn list_handler<'a>(selection:Vec<usize>,values:Vec<&'a str>)->ZArray
// {
//     let mut arr:ZArray = ZArray::new();
//     for i in selection.into_iter() {
//         arr.insert(i as u64, ZVal::from(values[i]));
//     }
//     arr
// }




// fn method_test<'a>(arguments:&'a mut [ZVal])->Result<(), phper::Error>
// {
//     // let mutarg_list = arguments.iter();
//     // if let (Some(promps),Some(list),Some(theme),None) = (arg_list.next(),arg_list.next(),arg_list.next(),arg_list.next()){
//     //     let promps:&'a str = promps.expect_z_str()?.to_str()?;
//     //     let list:Vec<&'a str> = list_maker(list)?;
//     //     let mut stdout:Term = Term::buffered_stdout();
//     //     stdout.execute(crossterm::cursor::MoveTo(5,5)).unwrap();
//     //     stdout.write_line("[Space] select | [Enter] valid | [↑↓] navigate | [a] select all").unwrap();
//     //     let theme = theme.expect_mut_z_obj()?.call(&"value",[]).unwrap();
//     //     let selection:Vec<usize> = MultiSelect::with_theme(theme).with_prompt(promps).items(&list.clone()).interact_on(&stdout).unwrap();
//     //     let arr:ZArray = list_handler(selection, list);
//     //     Ok(())
//     // }else {
//     //     Ok(())
//     //     // method_multi_select(arguments)
//     // }
//     // let mut arg_list = arguments.iter();
//     // let (Some(theme),Some(theme),None) = (arg_list.next(),arg_list.next(),arg_list.next()) else {
//     //     return Ok(());
//     // };
    
//     let value = arguments[0].expect_mut_z_obj()?.get_class();
//     dbg!(value);
//     Ok(())
// }

// fn method_password<'a>(arguments:&'a mut [ZVal])-> Result<(),phper::Error>
// {
//     let promps:&'a str = arguments[0].expect_z_str()?.to_str()?;
//     let password = Password::new()
//     .with_prompt(promps)
//     .with_confirmation("confirm password", "Passwords mismatching")
//     .interact().unwrap();
//     dbg!(password);
//     Ok(())
// }



