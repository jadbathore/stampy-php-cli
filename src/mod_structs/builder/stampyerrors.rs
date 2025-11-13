use phper::{
    classes::{ClassEntity, StateClass, Visibility},
    errors::ArgumentCountError, 
    functions::{Argument, MethodEntity}, 
    objects::StateObj, types::ArgumentTypeHint, values::ZVal 
};


use crate::{mod_structs::error_handler::StampyKindHandler, mod_traits};

use mod_traits::builder::class::BuilderClass;

#[derive(Default)]
pub struct StampyExceptionBuilder<T:'static>
{
    class:Option<ClassEntity<T>>
}


impl<'b> StampyExceptionBuilder<StampyKindHandler> 
where 
    'static:'b
{
    fn constructor(this:&mut StateObj<StampyKindHandler>,arguments:&mut [ZVal])->Result<(),phper::Error>
    {
        Self::preformate(arguments, |message,file,case|{
            this.set_property("message", message.to_owned());
            this.set_property("file", file.to_owned());
            let state_enum = this.as_mut_state();
            state_enum.try_set_case(case)?;
            Ok(())
        })?;
        Ok(())
    }
    fn get_format_message(this:&mut StateObj<StampyKindHandler>,_:&mut [ZVal])->Result<(),phper::Error>
    {
        
        let binder = this.as_state();
        let message = this.get_property("message");
        let file = this.get_property("file");
        binder.get_type_error()?.format_message(&mut [message,file])?;
        Ok(())

    }

    fn preformate<B>(arguments:&mut [ZVal],builder:B) -> Result<(),phper::Error>
    where 
        B: FnOnce(&ZVal,&ZVal,&str)-> Result<(),phper::Error>
    {
        let mut arg_list = arguments.iter();
        let arguments_expected  = (arg_list.next(),arg_list.next(),arg_list.next(),arg_list.next());
        if let (Some(message),Some(file),Some(case),None) = arguments_expected {
            let str = case.expect_z_str()?.to_str()?;
            builder(message,file,str)?;
            Ok(())
        } else {
            Err(phper::Error::ArgumentCount(ArgumentCountError::new(String::from("push"), 3, arguments.iter().len())))
        }
    }  
}

impl<'a,T> StampyExceptionBuilder<T> {
    fn set_arguments(method_entity:&'a mut MethodEntity){
        method_entity
        .argument(Argument::new("message").with_type_hint(ArgumentTypeHint::String))
        .argument(Argument::new("file").with_type_hint(ArgumentTypeHint::String))
        .argument(Argument::new("case").with_type_hint(ArgumentTypeHint::String));
    }
}

impl BuilderClass for StampyExceptionBuilder<StampyKindHandler>
{
    const CLASS_NAME:&str = "StampyException";
    type OutputType = ClassEntity<StampyKindHandler>;

    fn set_class(&mut self){ 
        let mut class_entity = ClassEntity::new_with_state_constructor(Self::CLASS_NAME,StampyKindHandler::default);
        class_entity.extends(StateClass::from_name("Exception"));
        self.class = Some(class_entity);
        
    }

    fn set_methods(&mut self) {
        if let Some(class) = &mut self.class {
            Self::set_arguments(
            class.add_method("__construct", Visibility::Public,Self::constructor)
            );
            class.add_method("getFormatMessage", Visibility::Public,Self::get_format_message);

        }
    }

    fn build(self) -> Self::OutputType { 
        self.class.expect("no class supply")
    }
}