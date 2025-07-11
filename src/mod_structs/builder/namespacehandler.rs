
use phper::{
    arrays::{ZArr, ZArray}, classes::{ClassEntity,Visibility}, functions::{Argument,MethodEntity}, objects::{StateObj, ZObj}, types::ArgumentTypeHint, values::ZVal
};

use crate::
    {
        mod_enums::arguments::ArgumentUsageNamespaceHandler, mod_structs::namespace_buf::{self, ClassesInNamespace}, mod_traits
    };
use std::{ffi::OsStr, fs, io::Error, path::{self, PathBuf}};

use mod_traits::builder::class::BuilderPropertyClass;



#[derive(Default)]
pub struct NamespaceHandler<T:'static>
{
    class:Option<ClassEntity<T>>,
}

impl NamespaceHandler<()> {

    fn constructor(this:&mut StateObj<()>,arguments:&mut [ZVal])->Result<(),phper::Error>
    {
        let path_arg:&str = arguments[0].expect_z_str()?.to_str()?;
        let namespace_arg:&str = arguments[1].expect_z_str()?.to_str()?;
        let binding = namespace_arg.split("\\").map(OsStr::new).collect::<Vec<&OsStr>>();
        let namespace_pointer = binding.as_slice();
        let classes_in_namespace:ClassesInNamespace = ClassesInNamespace::from(namespace_pointer);
        let mut path_buf = PathBuf::new();
        path_buf.push(path_arg);
        let mut z_array = ZArray::new();
        classes_in_namespace.resolver(path_buf,&mut z_array);
        this.set_property("classes",z_array);
        this.set_property("namespace",namespace_arg);
        Ok(())
    }

    fn get_associted_class<'a>(this:&mut StateObj<()>,_:&mut [ZVal])->Result<ZArray, phper::Error>
    {
        let a = this.get_property("classes").expect_z_arr()?.to_owned();
        Ok(a)
    }

    fn get_namespace<'a>(this:&mut StateObj<()>,_:&mut [ZVal])->Result<String, phper::Error>
    {
        let a = this.get_property("namespace").expect_z_str()?.to_str()?;
        Ok(a.to_owned())
    }

    // fn add(this:&mut StateObj<()>,_:&mut [ZVal])->Result<ZArray, phper::Error>
    // {
    //     let a = this.get_property("namespace").expect_z_str()?.to_str()?;
    //     namespace_buf.push(namespace);
    // }

    // fn pop(this:&mut StateObj<()>,_:&mut [ZVal])->Result<ZArray, phper::Error>
    // {
    //     let a = this.get_property("namespace").expect_z_str()?.to_str()?;
    // }

    
}


impl<T> NamespaceHandler<T> {
    fn set_arguments(argument_usage:ArgumentUsageNamespaceHandler,method_entity:&mut MethodEntity){
        match argument_usage {
            ArgumentUsageNamespaceHandler::PathWithNameSpace => {
                method_entity.argument(Argument::new("path").with_type_hint(ArgumentTypeHint::String))
                .argument(Argument::new("namespace").with_type_hint(ArgumentTypeHint::String));
            }
            
        }
    }
    
}

impl BuilderPropertyClass for NamespaceHandler<()> 
{
    type OutputType = ClassEntity<()>;

    fn set_class(&mut self,class_name:&str) { 
        self.class = Some(ClassEntity::new(class_name));
    }

    fn set_property(&mut self) {
        if let Some(class) = &mut self.class {
            class.add_property("classes", Visibility::Private, ());
            class.add_property("namespace", Visibility::Private, ());
        }
    }

    fn set_methods(&mut self) {
        if let Some(class) = &mut self.class {
            Self::set_arguments(
                ArgumentUsageNamespaceHandler::PathWithNameSpace, 
                class.add_method("__construct", Visibility::Public,Self::constructor)
            );
            class.add_method("getAssocitedClass", Visibility::Public,Self::get_associted_class);
            class.add_method("getNamespace", Visibility::Public,Self::get_namespace);
        }
    }

    fn build(self) -> Self::OutputType { 
        self.class.expect("no class supply")
    }
}
