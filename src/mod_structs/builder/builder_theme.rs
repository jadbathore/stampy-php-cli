
use phper::{
    classes::{ClassEntity,Visibility}, 
    functions::{Argument, MethodEntity},
    values::{ZVal},
    types::{ArgumentTypeHint},
    arrays::ZArray
};
use dialoguer::{console::Term,MultiSelect,Input,Password,/*Confirm,*/Select};
use crate::
    {
        mod_enums, 
        mod_traits::{self, builder::builder_class::BuilderPropertyClass},
    };
use mod_enums::{
    arguments::ArgumentUsage,
    themes::Themes
};
use mod_traits::builder::builder_class::BuilderClass;

#[derive(Default)]
pub struct ThemeBuilder<T:'static>
{
    class:Option<ClassEntity<T>>,
    theme:Option<Themes<'static>>

}

impl ThemeBuilder<()> {
    
}


impl<T> ThemeBuilder<T> {
    fn set_arguments(argument_usage:ArgumentUsage,method_entity:&mut MethodEntity){
        match argument_usage {
            ArgumentUsage::StringWithOptionalTheme => {
                
            },
            ArgumentUsage::StringAndListWithOptionalTheme => {
                
            }
        }
    }
}

impl BuilderPropertyClass for ThemeBuilder<()> 
{
    type OutputType = ClassEntity<()>;

    fn set_class(&mut self,class_name:&str) { 
        self.class = Some(ClassEntity::new(class_name));
    }

    fn set_methods(&mut self) {
        if let Some(class) = &mut self.class {
            class;
        }
    }

    fn set_property(&mut self){
        if let Some(class) = &mut self.class {
            class.add_property("themeEntity",Visibility::Private,)
        }
    }

    fn build(self) -> Self::OutputType { 
        self.class.expect("no class supply")
    }
}