use crossterm::ExecutableCommand;
use phper::{
    classes::{ClassEntity,Visibility}, 
    functions::{Argument, MethodEntity},
    values::{ZVal},
    types::{ArgumentTypeHint},
    arrays::ZArray
};
use dialoguer::{console::Term, theme::{ColorfulTheme, Theme}, Confirm, Input, MultiSelect, Password, Select};
use crate::{mod_enums::{self, actions::Actions, themes},mod_traits};
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

    /// in php Dialoguer::confirm
    fn confirm(arguments:&mut [ZVal])->Result<bool, phper::Error>
    {
        let promps:&str = arguments[0].expect_z_str()?.to_str()?;
        let mut arg_list = arguments.iter();
        let (arg1,arg2,arg3) = (arg_list.next(),arg_list.next(),arg_list.next());
        let mut result:bool = false;
        if let (Some(_),Some(theme),None) = (arg1,arg2,arg3){
            let with_theme:bool = theme.expect_bool()?;
            let confirm = Self::with_custom_theme(with_theme,|theme| {
            if let Some(theme) = theme {
                Confirm::with_theme(&theme).with_prompt(promps).interact()
            } else {
                Confirm::new().with_prompt(promps).interact()
            }
            });
            result = confirm.unwrap();
        } else if let (Some(_),None)= (arg1,arg2) {
            let confirm:Confirm<'static> = Confirm::new();
            result = confirm.with_prompt(promps).interact().unwrap();
        }
        Ok(result)
    }

    /// in php Dialoguer::select
    fn select(arguments:&mut [ZVal])->Result<String, phper::Error>
    {
        let mut arg_list = arguments.iter();
        let promps:&str = arguments[0].expect_z_str()?.to_str()?;
        let (arg1,arg2,arg3,arg4) = (arg_list.next(),arg_list.next(),arg_list.next(),arg_list.next());
        let vec:Vec<String> = Self::list_maker(&arguments[1])?;
        let mut result:usize = 0;
        if let (Some(_),Some(_),Some(theme),None) = (arg1,arg2,arg3,arg4){
            let with_theme:bool = theme.expect_bool()?;
            let select = Self::with_custom_theme(with_theme,|theme| {
            if let Some(theme) = theme {
                Select::with_theme(&theme).with_prompt(promps).items(&vec).interact()
            } else {
                Select::new().with_prompt(promps).items(&vec).interact()
            }
            });
            result = select.unwrap()
        } else if let (Some(_),Some(_),None) = (arg1,arg2,arg3) {
            result = Select::new().with_prompt(promps).items(&vec).interact().unwrap();
        }
        Ok(vec[result].clone())
    }


    // // in php = Dialoguer::multiSelect
    fn multi_select(arguments:&mut [ZVal])->Result<ZArray, phper::Error>
    {
        let promps:&str = arguments[0].expect_z_str()?.to_str()?;
        let vec:Vec<String> = Self::list_maker(&arguments[1])?;
        let mut stdout:Term = Term::stdout();
        stdout.execute(crossterm::cursor::MoveTo(5,5)).unwrap();
        stdout.write_line("[Space] select | [Enter] valid | [↑↓] navigate | [a] select all").unwrap();
        let mut arg_list = arguments.iter();
        let (arg1,arg2,arg3,arg4) = (arg_list.next(),arg_list.next(),arg_list.next(),arg_list.next());
        let mut result:Vec<usize> = Vec::new();
        if let (Some(_),Some(_),Some(theme),None) = (arg1,arg2,arg3,arg4) {
            let with_theme:bool = theme.expect_bool()?;
            let select = Self::with_custom_theme(with_theme,|theme| {
            if let Some(theme) = theme {
                MultiSelect::with_theme(&theme).with_prompt(promps).items(&vec).interact_on(&stdout)
            } else {
                MultiSelect::new().with_prompt(promps).items(&vec).interact_on(&stdout)
            }
            });
            result = select.unwrap()
        } else if let (Some(_),Some(_),None) = (arg1,arg2,arg3) {
            result = MultiSelect::new().with_prompt(promps).items(&vec).interact_on(&stdout).unwrap();
        }

        let arr:ZArray = Self::list_handler(result, vec);
        Ok(arr) 
    }

    // in php = Dialoguer::password
    fn password(arguments:&mut [ZVal])-> Result<(),phper::Error>
    {
        let promps:&str = arguments[0].expect_z_str()?.to_str()?;
        let mut arg_list = arguments.iter();
        let (arg1,arg2,arg3) = (arg_list.next(),arg_list.next(),arg_list.next());
        let mut result:String;
        if let (Some(_),Some(theme),None) = (arg1,arg2,arg3) {
            let with_theme:bool = theme.expect_bool()?;
            let select = Self::with_custom_theme(with_theme,|theme| {
            if let Some(theme) = theme {
                Password::with_theme(&theme)
                .with_prompt(promps)
                .with_confirmation("confirm password", "Passwords mismatching")
                .interact()
            } else {
                Password::new()
                .with_prompt(promps)
                .with_confirmation("confirm password", "Passwords mismatching")
                .interact()
            }
            });
            result = select.unwrap()
        } else if let (Some(_),Some(_),None) = (arg1,arg2,arg3) {
            result = Password::new()
            .with_prompt(promps)
            .with_confirmation("confirm password", "Passwords mismatching")
            .interact().unwrap();
        }
        dbg!(result);
        Ok(())
    }

    fn list_maker(z_value:&ZVal)->Result<Vec<String>, phper::Error>
    {
        let mut vec:Vec<String> = Vec::new();
        let select_list = z_value.expect_z_arr()?.iter();
        for (_key,value) in select_list {
            let check_value:String = value.expect_z_str()?.to_str()?.to_string();
            vec.push(check_value);
        }
        Ok(vec)
    }

    fn list_handler(selection:Vec<usize>,values:Vec<String>)->ZArray
    {
        let mut arr:ZArray = ZArray::new();
        for i in selection.into_iter() {
            arr.insert(i as u64, ZVal::from(values[i].clone()));
        }
        arr
    }

    
    fn with_custom_theme<A,T>(with_theme:bool,builder: A) -> T
    where A: FnOnce(Option<ColorfulTheme>) -> T,
    {
        if with_theme {
            let theme = ColorfulTheme::default();
            builder(Some(theme))
        } else{ 
            builder(None)
        }
    }
}


impl<T> DialoguerBuilder<T> {
    fn set_arguments(argument_usage:ArgumentUsage,method_entity:&mut MethodEntity){
        match argument_usage {
            ArgumentUsage::StringWithOptionalTheme => {
                method_entity.argument(Argument::new("input").with_type_hint(ArgumentTypeHint::String))
                .argument(Argument::new("theme")
                .with_type_hint(ArgumentTypeHint::Bool)
                .optional());
            },
            ArgumentUsage::StringAndListWithOptionalTheme => {
                method_entity
                .argument(Argument::new("input").with_type_hint(ArgumentTypeHint::String))
                .argument(Argument::new("list").with_type_hint(ArgumentTypeHint::Array))
                .argument(
                    Argument::new("theme")
                    .with_type_hint(ArgumentTypeHint::Bool)
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
            Self::set_arguments(ArgumentUsage::StringWithOptionalTheme, class.add_static_method("confirm", Visibility::Public,Self::confirm));
            Self::set_arguments(ArgumentUsage::StringWithOptionalTheme, class.add_static_method("input",Visibility::Public, Self::input));
            Self::set_arguments(ArgumentUsage::StringAndListWithOptionalTheme, class.add_static_method("select", Visibility::Public, Self::select));
            Self::set_arguments(ArgumentUsage::StringAndListWithOptionalTheme, class.add_static_method("Multiselect", Visibility::Public, Self::multi_select));
            Self::set_arguments(ArgumentUsage::StringWithOptionalTheme, class.add_static_method("password",Visibility::Public, Self::password));
        }
    }

    fn build(self) -> Self::OutputType { 
        self.class.expect("no class supply")
    }
}