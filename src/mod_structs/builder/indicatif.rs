use std::borrow::Cow;
use indicatif::ProgressBar;
use phper::{
    classes::{ClassEntity,Visibility},
    errors::ThrowObject,
    functions::{Argument, MethodEntity},
    objects::StateObj, 
    types::ArgumentTypeHint,
    values::ZVal
};

use crate::{mod_enums, mod_traits,general};
use mod_enums::arguments::ArgumentUsageProgressBar;
use mod_traits::builder::class::BuilderClass;

#[derive(Default)]
pub struct ProgressBarBuilder
{
    class:Option<ClassEntity<ProgressBar>>,
}

impl ProgressBarBuilder {

    fn progress_closure()->ProgressBar
    {
        ProgressBar::new(0)
    }

    fn constructor(this:&mut StateObj<ProgressBar>,arguments:&mut [ZVal])->Result<(),phper::Error>
    {
        let class_mut = this.as_mut_state();
        Self::preformate_argument_length(arguments,ArgumentUsageProgressBar::Length, |length|{
            class_mut.inc_length(length);
            Ok(())
        })
    }


    fn increment(this:&mut StateObj<ProgressBar>,arguments:&mut [ZVal])->Result<(),phper::Error>
    {
        let class_mut = this.as_mut_state();
        Self::preformate_argument_optionalincrementation(arguments, |length|{
            class_mut.inc(length);
            Ok(())
        })
    }

    fn finish(this:&mut StateObj<ProgressBar>,_:&mut [ZVal])->Result<(),phper::Error>
    {
        let class_mut = this.as_mut_state();
        class_mut.finish();
        Ok(())
    }

    fn finish_and_clear(this:&mut StateObj<ProgressBar>,_:&mut [ZVal])->Result<(),phper::Error>
    {
        let class_mut = this.as_mut_state();
        class_mut.finish_and_clear();
        Ok(())
    }

    fn finish_and_message(this:&mut StateObj<ProgressBar>,arguments:&mut [ZVal])->Result<(),phper::Error>
    {
        let class_mut = this.as_mut_state();
        let message = arguments[0].expect_mut_z_str()?.to_str()?.to_owned();
        let cow_message:Cow<'static, str> = Cow::from(general::leak_value(message));
        class_mut.finish_with_message(cow_message);
        Ok(())
    }

    fn preformate_argument_length<B>(arguments:&mut [ZVal],argument_type:ArgumentUsageProgressBar,builder:B) -> Result<(),phper::Error>
        where 
            B: FnOnce(u64)-> Result<(),phper::Error>
    {
        let length = arguments[0].expect_long()?;
        let unsigned_length = Self::convert_to_unsigned(length,argument_type)?;
        builder(unsigned_length)
    }

    fn preformate_argument_optionalincrementation<B>(arguments:&mut [ZVal],builder:B) -> Result<(),phper::Error>
        where 
            B: FnOnce(u64)-> Result<(),phper::Error>
    {
        let mut arg_list = arguments.iter();
        if let Some(increment) = arg_list.next() {
            Self::preformate_argument_length(&mut [increment.to_owned()],ArgumentUsageProgressBar::OptionalIncrementation, builder)
        } else {
            builder(1)
        }
    }

    fn set_arguments(argument_usage:ArgumentUsageProgressBar,method_entity:&mut MethodEntity){
        match argument_usage {
            ArgumentUsageProgressBar::Length => {
                method_entity.argument(Argument::new("length").with_type_hint(ArgumentTypeHint::Int));
            },
            ArgumentUsageProgressBar::OptionalIncrementation => {
                method_entity.argument(Argument::new("increment")
                .with_type_hint(ArgumentTypeHint::Int)
                .with_default_value("1")
            );
            },
            ArgumentUsageProgressBar::Message =>{
                method_entity.argument(Argument::new("message")
                .with_type_hint(ArgumentTypeHint::String)
            );
            }

        }
    }

    

    fn convert_to_unsigned(length: i64,argument_type:ArgumentUsageProgressBar)-> Result<u64,ThrowObject>
    {
        u64::try_from(length)
        .map_err(|_|
            {
                let arg = {
                    match argument_type {
                        ArgumentUsageProgressBar::Length => String::from("Length"),
                        ArgumentUsageProgressBar::OptionalIncrementation => String::from("Increment"),
                        _ => "".to_owned()
                    }
                };
                let format_message = arg + " can't be negative found " + &length.to_string();
                general::format_throwable_error(&format_message).unwrap()
            }
        )
    }
}






impl BuilderClass for ProgressBarBuilder
{
    type OutputType = ClassEntity<ProgressBar>;
    const CLASS_NAME:&'static str = "Indicatif";

    fn set_class(&mut self) { 
        self.class = Some(ClassEntity::new_with_state_constructor(Self::CLASS_NAME,Self::progress_closure));
        
    }

    fn set_methods(&mut self) {
        if let Some(class) = &mut self.class {
            Self::set_arguments(
                ArgumentUsageProgressBar::Length,
                class.add_method("__construct", Visibility::Public, Self::constructor)
            );
            Self::set_arguments(
                ArgumentUsageProgressBar::OptionalIncrementation, 
                class.add_method("increment", Visibility::Public, Self::increment)
            );
            class.add_method("finish", Visibility::Public, Self::finish);
            class.add_method("finishAndClear", Visibility::Public, Self::finish_and_clear);
            class.add_method("__destruct", Visibility::Public, Self::finish_and_clear);
            Self::set_arguments(
                ArgumentUsageProgressBar::OptionalIncrementation, 
                class.add_method("finishAndMessage", Visibility::Public, Self::finish_and_message)
            );
        }
    }

    fn build(self) -> Self::OutputType { 
        self.class.expect("no class supply")
    }
}
