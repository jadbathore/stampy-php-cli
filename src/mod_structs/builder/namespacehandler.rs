
use phper::{
    classes::{ClassEntity,Visibility}, functions::{Argument,MethodEntity}, types::ArgumentTypeHint, values::ZVal
};

use crate::
    {
        mod_enums::arguments::ArgumentUsageNamespaceHandler, mod_structs::namespace_buf::NameSpaceBuf, mod_traits
    };
use std::{ffi::OsString,fs,path::PathBuf,io::Error};

use mod_traits::builder::class::BuilderClass;



#[derive(Default)]
pub struct NamespaceHandler<T:'static>
{
    class:Option<ClassEntity<T>>,
}

impl NamespaceHandler<()> {

    fn recursive_path<'a>(path:&'a str,namespace_full:&'a str,dir_entries:&mut Vec<OsString>)->Result<(),Error>
    {
        
        let binding = namespace_full.split("\\").collect::<Vec<&str>>();
        let namespace_pointer = binding.as_slice();
        let mut path_buf = PathBuf::new();
        path_buf.push(path);

        for i in 0..namespace_pointer.len(){
            let mut path_buf = path_buf.clone();
            path_buf.push(namespace_pointer[i]);
            if path_buf.exists() {
                let rests = &namespace_pointer[i+1..];
                for rest in rests {
                    path_buf.push(rest);
                } 
                for entry in fs::read_dir(path_buf)? {
                    let dir = entry?;
                    let mut path = dir.path();
                    path.set_extension("");
                    let mut namespace_buf:NameSpaceBuf = NameSpaceBuf::from(namespace_full);
                    namespace_buf.push(path.file_name());
                    dir_entries.push(namespace_buf.get_namespace());
                }
                break;
            }
        }
        Ok(())
    }

    fn get_associted_class(arguments: &mut [ZVal])->Result<(), phper::Error>
    {
        let path_arg:&str = arguments[0].expect_z_str()?.to_str()?;
        let namespace_arg:&str = arguments[1].expect_z_str()?.to_str()?;
        let mut vec:Vec<OsString> = Vec::new();
        Self::recursive_path(path_arg, namespace_arg,&mut vec)?;
        dbg!(vec);
        Ok(())
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
