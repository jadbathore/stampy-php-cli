use phper::{
    classes::{ClassEntity,Visibility}, functions::{Argument, MethodEntity}, sys::DIR, types::ArgumentTypeHint,
    values::{ZVal},
    arrays::ZArray
};

use dialoguer::MultiSelect;

use crate::
    {
        mod_enums, 
        mod_structs::entity::DialoguerEntity,
        mod_traits,
    };

use mod_enums::arguments::ArgumentUsage;
use mod_traits::builder::builder_class::BuilderClass;

pub struct ClassBuilder{}

impl<'a> BuilderClass<'a> for ClassBuilder{
    type OutputType = DialoguerEntity;

    fn set_input_and_optionnal_theme_args(&mut self,method_entity:&mut MethodEntity){
        method_entity
        .argument(Argument::new("input").with_type_hint(ArgumentTypeHint::String))
        .argument(Argument::new("theme")
        .with_type_hint(ArgumentTypeHint::ClassEntry(String::from(r"\Theme")))
        .optional());
    }
    fn set_input_list_and_optionnal_theme_args(&mut self,method_entity:&mut MethodEntity){
        method_entity
        .argument(Argument::new("input").with_type_hint(ArgumentTypeHint::String))
        .argument(Argument::new("list").with_type_hint(ArgumentTypeHint::Array))
        .argument(
            Argument::new("theme")
            .with_type_hint(ArgumentTypeHint::ClassEntry(String::from(r"\Theme")))
            .optional()
        );
    }

    

    fn build(&mut self) -> Self::OutputType
    {
        let mut class:ClassEntity<()> = ClassEntity::new("Dialoguer");
        self.set_input_and_optionnal_theme_args(
            class.add_static_method("select", Visibility::Public, BuilderClass::method_select)
        );
        self.set_input_list_and_optionnal_theme_args(
            class.add_static_method("input", Visibility::Public, DialoguerEntity::method_input)
        );


        DialoguerEntity::new(class)

        // self.add_method(
        //     ArgumentUsage::StringWithOptionalTheme,
        //     self.class.add_static_method("input", Visibility::Public, DialoguerEntity::method_input)
        // );
        // self.add_method(
        //     ArgumentUsage::StringAndListWithOptionalTheme, 
        // self.class.add_static_method("select", Visibility::Public, DialoguerEntity::method_select)
        // );
        // self.add_method(
        //     ArgumentUsage::StringAndListWithOptionalTheme, 
        // self.class.add_static_method("MultiSelect", Visibility::Public, DialoguerEntity::method_select)
        // );
        // self.add_method(
        //     ArgumentUsage::StringWithOptionalTheme, 
        // self.class.add_static_method("password", Visibility::Public, DialoguerEntity::method_password)
        // );
        // self.add_method(
        //     ArgumentUsage::StringWithOptionalTheme, 
        // self.class.add_static_method("confirm", Visibility::Public, DialoguerEntity::method_confirm)
        // );
        // DialoguerEntity::new(self.class.clone())
    }


    // in php = Dialoguer::confirm
    fn confirm(arguments:&mut [ZVal])->Result<bool, phper::Error>
    {
        let mut arg_list = arguments.iter();
        let mut confirm:Confirm;
        let promp:&str;
        if let (Some(promps),Some(theme),None) = (arg_list.next(),arg_list.next(),arg_list.next()){
            let value = theme.expect_mut_z_obj()?.get_property(&"value");
            let Theme = DialoguerTheme::from_str(value.expect_z_str()?.to_str()?);
            confirm = Confirm::with_theme(Theme.into())
        } else {
            confirm = Confirm::new()
        }
        let confirm = Confirm::new();
        let value:&str = arguments[1].expect_mut_z_obj()?.get_property(&"value")?;
        confirm.with_prompt(promps).interact().unwrap();
        Ok(confirm)
    }

    // in php = Dialoguer::input
    fn input(arguments:&'a mut [ZVal])->Result<String, phper::Error>
    {
        let promps:&'a str = arguments[0].expect_z_str()?.to_str()?;
        let input = Input::<String>::new()
        .with_prompt(promps).interact_text().unwrap();
        Ok(input)
    }

    // // in php = Dialoguer::select
    fn select(arguments:&'a mut [ZVal])->Result<String, phper::Error>
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

    // // in php = Dialoguer::multiSelect
    fn multi_select(arguments:&'a mut [ZVal])->Result<ZArray, phper::Error>
    {
        let promps:&'a str = arguments[0].expect_z_str()?.to_str()?;
        let vec:Vec<&'a str> = DialoguerEntity::list_maker(&arguments[1])?;
        let mut stdout:Term = Term::buffered_stdout();
        stdout.execute(crossterm::cursor::MoveTo(5,5)).unwrap();
        stdout.write_line("[Space] select | [Enter] valid | [↑↓] navigate | [a] select all").unwrap();
        let selection:Vec<usize> = MultiSelect::new().with_prompt(promps).items(&vec.clone()).interact_on(&stdout).unwrap();
        let arr:ZArray = DialoguerEntity::list_handler(selection, vec);
        Ok(arr)
    }

    // in php = DIaloguer::password
    fn password(arguments:&'a mut [ZVal])-> Result<(),phper::Error>
    {
        let promps:&'a str = arguments[0].expect_z_str()?.to_str()?;
        let password = Password::new()
        .with_prompt(promps)
        .with_confirmation("confirm password", "Passwords mismatching")
        .interact().unwrap();
        dbg!(password);
        Ok(())
    }

    fn method_mutli_select_handler(arguments:&'a mut [ZVal],multi_select_instance:MultiSelect)->Result<ZArray, phper::Error>
    {
        let promps:&'a str = arguments[0].expect_z_str()?.to_str()?;
        let vec:Vec<&'a str> = DialoguerEntity::list_maker(&arguments[1])?;
        let mut stdout:Term = Term::buffered_stdout();
        stdout.execute(crossterm::cursor::MoveTo(5,5)).unwrap();
        stdout.write_line("[Space] select | [Enter] valid | [↑↓] navigate | [a] select all").unwrap();
        let selection:Vec<usize> = multi_select_instance.with_prompt(promps).items(&vec.clone()).interact_on(&stdout).unwrap();
        let arr:ZArray = DialoguerEntity::list_handler(selection, vec);
        Ok(arr)
    }

    fn list_maker(z_value:&'a ZVal)->Result<Vec<&'a str>, phper::Error>
    {
        let mut vec:Vec<&'a str> = Vec::new();
        let select_list = z_value.expect_z_arr()?.iter();
        for (_key,value) in select_list {
            let check_value:&'a str = value.expect_z_str()?.to_str()?;
            vec.push(check_value);
        }
        Ok(vec)
    }

    fn list_handler(selection:Vec<usize>,values:Vec<&'a str>)->ZArray
    {
        let mut arr:ZArray = ZArray::new();
        for i in selection.into_iter() {
            arr.insert(i as u64, ZVal::from(values[i]));
        }
        arr
    }
}