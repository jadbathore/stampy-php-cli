
use phper::{
    arrays::ZArray, classes::{ClassEntity,Visibility}, errors::ArgumentCountError, functions::{Argument,MethodEntity}, objects::StateObj, types::ArgumentTypeHint, values::ZVal
};

use crate::
    {
        mod_enums::arguments::ArgumentUsageNamespaceHandler, mod_structs::namespace_buf::ClassesInNamespace, mod_traits
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
            .map(|x| Self::leak_value(x.to_owned()))
            .collect::<Vec<&'static str>>();
            let to_static_path:&'static str = Self::leak_value(path.to_owned());
            classes_in_namespace.extend_to_namespace(to_static_namespace.as_slice());
            classes_in_namespace.push_to_path(to_static_path);
            Ok(())
        })
    }

    fn leak_value(str:String)->&'static str
    {
        Box::leak(str.into_boxed_str())
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

    fn resolve(this:&mut StateObj<ClassesInNamespace<'b>>,_:&mut [ZVal])->Result<ZArray, phper::Error>
    {
        let class_in_namespace = this.as_mut_state();
        let a = class_in_namespace.resolver()?;
        Ok(a)
    }

    fn previous(this:&mut StateObj<ClassesInNamespace<'b>>,_:&mut [ZVal])->Result<(), phper::Error>
    {
        this.as_mut_state().pop();
        
        Ok(())
    }
}

impl<'a,T> NamespaceHandler<T> {
    fn set_arguments(argument_usage:ArgumentUsageNamespaceHandler,method_entity:&'a mut MethodEntity){
        match argument_usage {
            ArgumentUsageNamespaceHandler::PathWithNameSpace => {
                method_entity.argument(Argument::new("path").with_type_hint(ArgumentTypeHint::String))
                .argument(Argument::new("namespace").with_type_hint(ArgumentTypeHint::String));
            }
            
        }
    }
}

impl<'a> BuilderClass for NamespaceHandler<ClassesInNamespace<'a>> 
{
    type OutputType = ClassEntity<ClassesInNamespace<'a>>;

    fn set_class(&mut self,class_name:&str) { 
        self.class = Some(ClassEntity::new_with_state_constructor(class_name, ClassesInNamespace::new));
    }

    fn set_methods(&mut self) {
        if let Some(class) = &mut self.class {
            Self::set_arguments(
                ArgumentUsageNamespaceHandler::PathWithNameSpace, 
                class.add_method("__construct", Visibility::Public,Self::constructor)
            );
            class.add_method("resolve", Visibility::Public,Self::resolve);
            class.add_method("previous", Visibility::Public, Self::previous);
        }
    }

    fn build(self) -> Self::OutputType { 
        self.class.expect("no class supply")
    }
}
