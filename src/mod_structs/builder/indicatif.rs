
use indicatif::ProgressBar;
use phper::{
    arrays::ZArray, classes::{ClassEntity,Visibility}, 
    errors::{ArgumentCountError}, 
    objects::StateObj,
    functions::{Argument, MethodEntity}, types::ArgumentTypeHint, values::ZVal
};
use dialoguer::{ theme::{ColorfulTheme}, Confirm, Editor, Input, MultiSelect, Password, Select};
use crate::{mod_enums,mod_traits::{self, allow}};
use mod_enums::arguments::ArgumentUsageProgressBar;
use mod_traits::builder::class::BuilderClass;

#[derive(Default)]
pub struct IndicatifBuilder
{
    class:Option<ClassEntity<ProgressBar>>
}

impl IndicatifBuilder {
    fn constructor(this:&mut StateObj<ProgressBar>,arguments:&mut [ZVal])->Result<(),phper::Error>
    {
        let progressbar:&mut ProgressBar = this.as_mut_state();
        dbg!(progressbar.length());
        Ok(())
    }
}

impl IndicatifBuilder {
    // fn set_arguments(argument_usage:ArgumentUsageProgressBar,method_entity:&mut MethodEntity){
    //     match argument_usage {
            
    //     }
    // }
}

impl BuilderClass for IndicatifBuilder
{
    type OutputType = ClassEntity<ProgressBar>;

    fn set_class(&mut self,class_name:&str) { 
        self.class = Some(ClassEntity::new_with_state_constructor(class_name,ProgressBar::no_length));
    }

    fn set_methods(&mut self) {
        if let Some(class) = &mut self.class {
            class.add_method("__construct", Visibility::Public, Self::constructor);
        }
    }

    fn build(self) -> Self::OutputType { 
        self.class.expect("no class supply")
    }
}