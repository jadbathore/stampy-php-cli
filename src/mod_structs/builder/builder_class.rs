use crossterm::ExecutableCommand;
use phper::{
    classes::{ClassEntity,Visibility}, 
    functions::{Argument, MethodEntity},
    values::{ZVal},
    types::{ArgumentTypeHint},
    arrays::ZArray
};
use dialoguer::{console::Term,MultiSelect,Input,Password,/*Confirm,*/Select};
use crate::{mod_enums,mod_traits};
use mod_enums::arguments::ArgumentUsage;
use mod_traits::builder::builder_class::BuilderClass;

#[derive(Default)]
pub struct DialoguerBuilder<T:'static>
{
    class:Option<ClassEntity<T>>
}

impl DialoguerBuilder<()> {
    fn input(arguments:&mut [ZVal])->Result<String, phper::Error>
    {
        let promps:&str = arguments[0].expect_z_str()?.to_str()?;
        let input = Input::<String>::new()
        .with_prompt(promps).interact_text().unwrap();
        Ok(input)
    }

    // fn confirm(arguments:&mut [ZVal])->Result<bool, phper::Error>
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

    fn select(arguments:&mut [ZVal])->Result<String, phper::Error>
    {
        let promps:&str = arguments[0].expect_z_str()?.to_str()?;
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
    fn multi_select(arguments:&mut [ZVal])->Result<ZArray, phper::Error>
    {
        let promps:&str = arguments[0].expect_z_str()?.to_str()?;
        let vec:Vec<&str> = Self::list_maker(&arguments[1])?;
        let mut stdout:Term = Term::stdout();
        stdout.execute(crossterm::cursor::MoveTo(5,5)).unwrap();
        stdout.write_line("[Space] select | [Enter] valid | [↑↓] navigate | [a] select all").unwrap();
        let selection:Vec<usize> = MultiSelect::new().with_prompt(promps).items(&vec.clone()).interact_on(&stdout).unwrap();
        let arr:ZArray = Self::list_handler(selection, vec);
        Ok(arr) 
    }

    // in php = Dialoguer::password
    fn password(arguments:&mut [ZVal])-> Result<(),phper::Error>
    {
        let promps:&str = arguments[0].expect_z_str()?.to_str()?;
        let password = Password::new()
        .with_prompt(promps)
        .with_confirmation("confirm password", "Passwords mismatching")
        .interact().unwrap();
        dbg!(password);
        Ok(())
    }

    fn list_maker(z_value:&ZVal)->Result<Vec<&str>, phper::Error>
    {
        let mut vec:Vec<&str> = Vec::new();
        let select_list = z_value.expect_z_arr()?.iter();
        for (_key,value) in select_list {
            let check_value:&str = value.expect_z_str()?.to_str()?;
            vec.push(check_value);
        }
        Ok(vec)
    }

    fn list_handler(selection:Vec<usize>,values:Vec<&str>)->ZArray
    {
        let mut arr:ZArray = ZArray::new();
        for i in selection.into_iter() {
            arr.insert(i as u64, ZVal::from(values[i]));
        }
        arr
    }
}


impl<T> DialoguerBuilder<T> {
    fn set_arguments(argument_usage:ArgumentUsage,method_entity:&mut MethodEntity){
        match argument_usage {
            ArgumentUsage::StringWithOptionalTheme => {
                method_entity.argument(Argument::new("input").with_type_hint(ArgumentTypeHint::String))
                .argument(Argument::new("theme")
                .with_type_hint(ArgumentTypeHint::ClassEntry(String::from(r"\Theme")))
                .optional());
            },
            ArgumentUsage::StringAndListWithOptionalTheme => {
                method_entity
                .argument(Argument::new("input").with_type_hint(ArgumentTypeHint::String))
                .argument(Argument::new("list").with_type_hint(ArgumentTypeHint::Array))
                .argument(
                    Argument::new("theme")
                    .with_type_hint(ArgumentTypeHint::ClassEntry(String::from(r"\Theme")))
                    .optional()
                );
            }
        }
    }
}

impl BuilderClass for DialoguerBuilder<()> 
{
    type OutputType = ClassEntity<()>;

    fn set_class(&mut self,class_name:&str) { 
        self.class = Some(ClassEntity::new(class_name));
    }

    fn set_methods(&mut self) {
        if let Some(class) = &mut self.class {
            Self::set_arguments(ArgumentUsage::StringWithOptionalTheme, class.add_static_method("input",Visibility::Public, Self::input));
            // Self::set_arguments(ArgumentUsage::StringWithOptionalTheme, class.add_static_method("confirm", Visibility::Public,Self::confirm));
            Self::set_arguments(ArgumentUsage::StringAndListWithOptionalTheme, class.add_static_method("select", Visibility::Public, Self::select));
            Self::set_arguments(ArgumentUsage::StringAndListWithOptionalTheme, class.add_static_method("Multiselect", Visibility::Public, Self::multi_select));
            Self::set_arguments(ArgumentUsage::StringWithOptionalTheme, class.add_static_method("password",Visibility::Public, Self::password));
        }
    }

    fn build(self) -> Self::OutputType { 
        self.class.expect("no class supply")
    }
}