
use phper::{
    arrays::ZArray, classes::{ClassEntity,Visibility}, errors::{ArgumentCountError}, functions::{Argument, MethodEntity}, types::ArgumentTypeHint, values::ZVal
};
use dialoguer::{ theme::{ColorfulTheme}, Confirm, Editor, Input, MultiSelect, Password, Select};
use crate::{mod_enums,mod_traits::{self, allow}};
use mod_enums::arguments::ArgumentUsageDialoguer;
use mod_traits::builder::class::BuilderClass;

#[derive(Default)]
pub struct DialoguerBuilder<T:'static>
{
    class:Option<ClassEntity<T>>
}

impl DialoguerBuilder<()> {

    /// in php Dialoguer::select
    fn select(arguments:&mut [ZVal])->Result<String, phper::Error>
    {
        Self::preformate_input_list_optionnal_theme(arguments, "select",
        |input,list,theme|{
            let result:usize;
            let select:Select;
            if let Some(theme) = theme {
                select = Select::with_theme(theme);
            } else {
                select = Select::new();
            }
            result = select.with_prompt(input).items(&list).interact()?;
            Ok(list[result].clone())
        })
    }


    // // in php = Dialoguer::multiSelect
    fn multi_select(arguments:&mut [ZVal])->Result<ZArray, phper::Error>
    {
        Self::preformate_input_list_optionnal_theme(arguments, "multiSelect",
        |input,list,theme|{
            let result:Vec<usize>;
            let multi_select:MultiSelect;
            if let Some(theme) = theme {
                multi_select = MultiSelect::with_theme(theme);
            } else {
                multi_select = MultiSelect::new();
            }
            result = multi_select.with_prompt(input).items(&list).interact()?;
            let z_array = Self::list_handler(result, list);
            Ok(z_array)
        })
    }

    /// in php = Dialoguer::input
    fn input(arguments:&mut [ZVal])->Result<String, phper::Error>
    {
        Self::preformate_input_optionnal_theme(arguments,"input" ,|prompt,theme|{
            let input:Input<'_,String>;
            if let Some(theme) = theme {
                input = Input::<String>::with_theme(theme);
            } else {
                input = Input::<String>::new();
            }
            let a = input.with_prompt(prompt).interact()?;
            Ok(a)
        })
    }

    /// in php = Dialoguer::password
    fn password(arguments:&mut [ZVal])-> Result<String,phper::Error>
    {
        Self::preformate_input_optionnal_theme(arguments, "password",|input,theme|{
            let password:Password;
            if let Some(theme) = theme {
                password = Password::with_theme(theme);
            } else {
                password = Password::new();
            }
            let a = password.with_prompt(input)
            .with_confirmation("confirm password", "Passwords mismatching")
            .interact()?;
            Ok(a)
        })
    }

    /// in php Dialoguer::confirm
    fn confirm(arguments:&mut [ZVal])->Result<bool, phper::Error>
    {
        Self::preformate_input_optionnal_theme(arguments, "confirm",|input,theme|{
            let confirm:Confirm;
            if let Some(theme) = theme {
                confirm = Confirm::with_theme(theme);
            } else {
                confirm = Confirm::new();
            }
            let a = confirm.with_prompt(input).interact()?;
            Ok(a)
        })
    }

    /// in php Dialoguer::editor
    fn editor(arguments:&mut [ZVal])->Result<(),phper::Error>
    {
        let promps:&str = arguments[0].expect_z_str()?.to_str()?;
        if let Some(rv) = Editor::new().edit(promps).unwrap() {
            println!("Your message:");
            println!("{}", rv);
        } else {
            println!("Aborted!");
        }
        Ok(())
    }

    fn error_mapper(error:impl std::error::Error+ 'static) -> phper::Error
    {
        phper::Error::Boxed(Box::new(error))
    }

    fn preformate_input_optionnal_theme<T: allow::AllowedForBoolAndString ,B>(arguments:&mut [ZVal],method_name:&str,builder:B) -> Result<T,phper::Error>
        where 
            B: FnOnce(&str,Option<&ColorfulTheme>)-> Result<T,dialoguer::Error>
    {
        let mut arg_list = arguments.iter();
        let arguments_expected  = (arg_list.next(),arg_list.next(),arg_list.next());
        if let (Some(input),None,None) = arguments_expected {
            let input_arg = input.expect_z_str()?.to_str()?;
            builder(input_arg,None)
            .map_err(|x|{
                Self::error_mapper(x)
            })
        } else if let (Some(input),Some(theme),None) = arguments_expected {
            let  input_arg = input.expect_z_str()?.to_str()?;
            let path_arg = theme.expect_bool()?;
            if path_arg {
                let theme:ColorfulTheme = ColorfulTheme::default();
                builder(input_arg,Some(&theme)).map_err(|x|{
                    Self::error_mapper(x)
                })
            } else {
                builder(input_arg,None).map_err(|x|{
                    Self::error_mapper(x)
                })
            }
        } else {
            Err(phper::Error::ArgumentCount(ArgumentCountError::new(String::from(method_name), 2, arguments.iter().len())))
        }
    }

    fn preformate_input_list_optionnal_theme<T: allow::AllowedForZArrayAndString ,B>(arguments:&mut [ZVal],method_name:&str,builder:B) -> Result<T,phper::Error>
        where 
            B: FnOnce(&str,Vec<String>,Option<&ColorfulTheme>)-> Result<T,dialoguer::Error>
    {
        let mut arg_list = arguments.iter();
        let promps:&str = arguments[0].expect_z_str()?.to_str()?;
        let vec:Vec<String> = Self::list_maker(&arguments[1])?;
        let (arg1,arg2,arg3,arg4) = (arg_list.next(),arg_list.next(),arg_list.next(),arg_list.next());
        if let (Some(_),Some(_),Some(theme),None) = (arg1,arg2,arg3,arg4){
            let path_arg = theme.expect_bool()?;
            if path_arg {
                let theme:ColorfulTheme = ColorfulTheme::default();
                builder(promps,vec,Some(&theme)).map_err(|x|{
                    Self::error_mapper(x)
                })
            } else {
                builder(promps,vec,None).map_err(|x|{
                    Self::error_mapper(x)
                })
            }
        } else if let (Some(_),Some(_),None,None) = (arg1,arg2,arg3,arg4) {
            builder(promps,vec,None).map_err(|x|{
                    Self::error_mapper(x)
            })
        } else {
            Err(phper::Error::ArgumentCount(ArgumentCountError::new(String::from(method_name), 2, arguments.iter().len())))
        }
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
}


impl<T> DialoguerBuilder<T> {
    fn set_arguments(argument_usage:ArgumentUsageDialoguer,method_entity:&mut MethodEntity){
        match argument_usage {
            ArgumentUsageDialoguer::StringWithOptionalTheme => {
                method_entity.argument(Argument::new("input").with_type_hint(ArgumentTypeHint::String))
                .argument(Argument::new("theme")
                .with_type_hint(ArgumentTypeHint::Bool)
                .optional());
            },
            ArgumentUsageDialoguer::StringAndListWithOptionalTheme => {
                method_entity
                .argument(Argument::new("input").with_type_hint(ArgumentTypeHint::String))
                .argument(Argument::new("list").with_type_hint(ArgumentTypeHint::Array))
                .argument(
                    Argument::new("theme")
                    .with_type_hint(ArgumentTypeHint::Bool)
                    .optional()
                );
            }
            ArgumentUsageDialoguer::String => {
                method_entity.argument(Argument::new("input").with_type_hint(ArgumentTypeHint::String));
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
            Self::set_arguments(ArgumentUsageDialoguer::String, class.add_static_method("editor", Visibility::Public,Self::editor));
            Self::set_arguments(ArgumentUsageDialoguer::StringWithOptionalTheme, class.add_static_method("confirm", Visibility::Public,Self::confirm));
            Self::set_arguments(ArgumentUsageDialoguer::StringWithOptionalTheme, class.add_static_method("input",Visibility::Public, Self::input));
            Self::set_arguments(ArgumentUsageDialoguer::StringAndListWithOptionalTheme, class.add_static_method("select", Visibility::Public, Self::select));
            Self::set_arguments(ArgumentUsageDialoguer::StringAndListWithOptionalTheme, class.add_static_method("multiSelect", Visibility::Public, Self::multi_select));
            Self::set_arguments(ArgumentUsageDialoguer::StringWithOptionalTheme, class.add_static_method("password",Visibility::Public, Self::password));
        }
    }

    fn build(self) -> Self::OutputType { 
        self.class.expect("no class supply")
    }
}