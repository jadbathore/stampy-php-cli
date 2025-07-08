
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
use std::{ffi::{OsStr, OsString}, fs::{DirEntry, ReadDir}, io::ErrorKind, path::Path};
use std::path::PathBuf;
use mod_traits::builder::builder_class::BuilderClass;
use std::{fs, io::Error};

#[derive(Default)]
pub struct NamespaceHandler<T:'static>
{
    class:Option<ClassEntity<T>>,
}

impl NamespaceHandler<()> {
    fn recursive_path<'a,'b>(path:&'a str,namespace_full:&'a str)->Result<Option<Vec<OsString>>,Error>
    {
        let namespace = namespace_full.split("\\").collect::<Vec<&str>>().into_iter().last();
        if let Some(namespace) = namespace {
            
            let mut vec:Vec<OsString> = Vec::new();
                    match fs::read_dir::<&str>(path)?.into_iter() {
                        mut iter => loop {
                            match iter.next() {
                                None => break,
                                Some(entry) => { 
                                    let dir:DirEntry = entry?;
                                    if dir.file_name() == namespace {
                                        for entry in fs::read_dir::<PathBuf>(dir.path())? {
                                            let dir:DirEntry = entry?;
                                            let mut file = dir.path();
                                            file.set_extension("");
                                            let namespace = OsStr::new(namespace_full);
                                            let mut result = OsString::from(namespace);
                                            let backslash = OsStr::new("\\");
                                            result.push(backslash);
                                            result.push(file.file_name().unwrap());
                                            vec.push(result);
                                            dbg!();
                                        }
                                    }
                                },
                            };
                        },
                    };
            Ok(Some(vec))
        } else {
            Ok(None)
        }
    }

    fn get_associted_class(arguments: &mut [ZVal])->Result<(), phper::Error>
    {
        let path_arg:&str = arguments[0].expect_z_str()?.to_str()?;
        let namespace_arg:&str = arguments[1].expect_z_str()?.to_str()?;
        
        let test:Option<Vec<OsString>> = Self::recursive_path(path_arg, namespace_arg)?;
        dbg!(test);
        // let path_vec:Vec<&str> = path.split("/").collect();
        // let path_iter =  path_vec.into_iter();
        // let namespace_iter = namespace_vec.into_iter();

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