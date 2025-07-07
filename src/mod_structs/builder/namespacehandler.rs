
use phper::{
    classes::{ClassEntity,Visibility}, 
    functions::{Argument,MethodEntity}, 
    values::ZVal,
    types::ArgumentTypeHint,
};

use crate::
    {
        mod_enums::arguments::{ArgumentUsageNamespaceHandler}, 
        mod_traits
    };

use mod_traits::builder::builder_class::BuilderClass;
use std::{io::Error, fs::{self, DirEntry}};

#[derive(Default)]
pub struct NamespaceHandler<T:'static>
{
    class:Option<ClassEntity<T>>,
}

impl NamespaceHandler<()> {
    const fn recursive_path<'a>(path:&'a str,actualNamespace:&'a str)->Result<(),Error> {
        for entry in fs::read_dir(path)? {
            let dir = entry?;

            if {
                println!("{:?},{:?}", dir.path(),a);
                break;
            }
        }
        Ok(())
    }

    fn get_associted_class(arguments: &mut [ZVal])->Result<(), phper::Error>
    {
        let path:&str = arguments[0].expect_z_str()?.to_str()?;
        let namespace:&str = arguments[1].expect_z_str()?.to_str()?;
        let parts: Vec<&str> = namespace.split('\\').collect();
        let mut slice_iter = parts.iter();
        let (_,b) = (slice_iter.next(),slice_iter.next());
       

        
        Ok::<(), phper::Error>(())
    }

}


impl<T> NamespaceHandler<T> {
    fn set_arguments(argument_usage:ArgumentUsageNamespaceHandler,method_entity:&mut MethodEntity){
        match argument_usage {
            ArgumentUsageNamespaceHandler::PathWithNameSpace => {
                method_entity.argument(Argument::new("path").with_type_hint(ArgumentTypeHint::String));
                // .argument(Argument::new("namespace").with_type_hint(ArgumentTypeHint::String));
            }
        }
    }
}

impl BuilderClass for NamespaceHandler<()> 
{
    type OutputType = ClassEntity<()>;

    fn set_class(&mut self,class_name:&str) { 
        self.class = Some(ClassEntity::new(class_name));
    }

    fn set_methods(&mut self) {
        if let Some(class) = &mut self.class {
            Self::set_arguments(
                ArgumentUsageNamespaceHandler::PathWithNameSpace, 
                class.add_static_method("getAssocitedClass", Visibility::Public,Self::get_associted_class)
            );
        }
    }
    

    fn build(self) -> Self::OutputType { 
        self.class.expect("no class supply")
    }
}