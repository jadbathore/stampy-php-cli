
use std::ffi::OsString;

use phper::{
    arrays::ZArray, classes::{ClassEntity,Visibility},
    errors::ArgumentCountError, functions::{Argument,MethodEntity}, 
    objects::StateObj, types::ArgumentTypeHint, values::ZVal
};

use crate::
    {
        general, mod_enums::{arguments::ArgumentUsageNamespaceHandler, errors::class_error::StampyErrorKind}, 
        mod_structs::namespace_buf::ClassesInNamespace, mod_traits
    };


use mod_traits::builder::class::BuilderClass;


#[derive(Default)]
pub struct NamespaceHandler<T:'static>
{
    class:Option<ClassEntity<T>>,
}

impl<'b> NamespaceHandler<ClassesInNamespace<'b>> 
where 
    'static:'b
{

    fn constructor(this:&mut StateObj<ClassesInNamespace<'b>>,arguments:&mut [ZVal])->Result<(),phper::Error>
    {
        Self::preformate_arguments(arguments,|path,namespace|{
            let classes_in_namespace = this.as_mut_state();
            let to_static_namespace = namespace.split("\\")
            .map(|x| general::leak_value(x.to_owned()))
            .collect::<Vec<&'static str>>();
            let to_static_path:&'static str = general::leak_value(path.to_owned());
            classes_in_namespace.extend_to_namespace(to_static_namespace.as_slice());
            classes_in_namespace.push_to_path(to_static_path);
            Ok(())
        })
    }

    fn resolve(this:&mut StateObj<ClassesInNamespace<'b>>,_:&mut [ZVal])->Result<ZArray, phper::Error>
    {
        let class_in_namespace = this.as_mut_state();
        let try_z_array = class_in_namespace.resolver();
        if let Err((path_buf,err)) = try_z_array {
            let throwable = general::format_throwable_exception(path_buf,err,StampyErrorKind::EmptyNameSpace)?;
            return Err(phper::Error::Throw(throwable));
        }
            Ok(try_z_array.unwrap())
    }

    fn previous(this:&mut StateObj<ClassesInNamespace<'b>>,_:&mut [ZVal])->Result<(), phper::Error>
    {
        this.as_mut_state().pop();
        Ok(())
    }

    fn push(this:&mut StateObj<ClassesInNamespace<'b>>,arguments:&mut [ZVal])->Result<(), phper::Error>
    {
        let state_class = this.as_mut_state();
        Self::preformate_namespace_slice(arguments, |namespace_slice|{

            if state_class.try_push(&namespace_slice)? {
                let leak_namespace = general::leak_value(namespace_slice.to_string_lossy().to_string());
                state_class.push_to_namespace(leak_namespace);
            Ok(())
            } else {
                let format_message = ["namespace ",&state_class.get_namespace(),"\\",namespace_slice.to_str().unwrap()," don't exist"];
                let throwable = general::format_throwable_error(&format_message.join(""))?;
                Err(phper::Error::Throw(throwable))
            }
        })
    }

    


    fn preformate_arguments<B>(arguments:&mut [ZVal],builder:B) -> Result<(),phper::Error>
    where 
        B: FnOnce(&str,&str)-> Result<(),phper::Error>
    {
        let mut arg_list = arguments.iter();
        let arguments_expected  = (arg_list.next(),arg_list.next(),arg_list.next());
        if let (Some(path),Some(namespace),None) = arguments_expected {
            let path_arg = path.expect_z_str()?.to_str()?;
            let namespace_arg= namespace.expect_z_str()?.to_str()?;
            builder(path_arg,namespace_arg)?;
            Ok(())
        } else {
            Err(phper::Error::ArgumentCount(ArgumentCountError::new(String::from("__contructor"), 2, arguments.iter().len())))
        }
    }  

    fn preformate_namespace_slice<B>(arguments:&mut [ZVal],builder:B) -> Result<(),phper::Error>
    where 
        B: FnOnce(OsString)-> Result<(),phper::Error>
    {
        let mut arg_list = arguments.iter();
        let arguments_expected  = (arg_list.next(),arg_list.next());
        if let (Some(path),None) = arguments_expected {
            let path_arg = path.expect_z_str()?.to_str()?;
            builder(OsString::from(path_arg))?;
            Ok(())
        } else {
            Err(phper::Error::ArgumentCount(ArgumentCountError::new(String::from("push"), 1, arguments.iter().len())))
        }
    }  
}

impl<'a,T> NamespaceHandler<T> {
    fn set_arguments(argument_usage:ArgumentUsageNamespaceHandler,method_entity:&'a mut MethodEntity){
        match argument_usage {
            ArgumentUsageNamespaceHandler::PathWithNameSpace => {
                method_entity.argument(Argument::new("path").with_type_hint(ArgumentTypeHint::String))
                .argument(Argument::new("namespace").with_type_hint(ArgumentTypeHint::String));
            },
            ArgumentUsageNamespaceHandler::NamespaceSlice => {
                method_entity.argument(Argument::new("namespaceSlice").with_type_hint(ArgumentTypeHint::String));
            }
            
        }
    }
}

impl<'a> BuilderClass for NamespaceHandler<ClassesInNamespace<'a>> 
{
    const CLASS_NAME:&'static str = "NamespaceHandler";
    type OutputType = ClassEntity<ClassesInNamespace<'a>>;

    fn set_class(&mut self) { 
        self.class = Some(ClassEntity::new_with_state_constructor(Self::CLASS_NAME, ClassesInNamespace::new));
    }

    fn set_methods(&mut self) {
        if let Some(class) = &mut self.class {
            Self::set_arguments(
                ArgumentUsageNamespaceHandler::PathWithNameSpace, 
                class.add_method("__construct", Visibility::Public,Self::constructor)
            );
            class.add_method("resolve", Visibility::Public,Self::resolve);
            class.add_method("previous", Visibility::Public, Self::previous);
            class.add_method("push", Visibility::Public, Self::push);
        }
    }

    fn build(self) -> Self::OutputType { 
        self.class.expect("no class supply")
    }
}
