use phper::{classes::StateClass,errors::InitializeObjectError};
use crate::mod_enums::errors::class_error::{GlobalHandlerError, StampyErrorKind};


#[derive(Default)]
pub struct StampyKindHandler
{
    error_type:Option<StampyErrorKind>
}


impl StampyKindHandler
{
    pub fn try_set_case(&mut self,case:&str)->Result<(),phper::Error>
    {
        let bind = StateClass::from_name("StampyErrorKind");
        let class_enum = bind.as_class_entry();
        if let Some(_) = class_enum.get_static_property(case) {
                let kind = Self::match_kind(case)?;
                self.error_type = Some(kind);
                Ok(())
        } else {
            let obj_err = InitializeObjectError::new("unknown case ".to_string());
            return Err(phper::Error::InitializeObject(obj_err));
        }
    }

    fn match_kind(kind:&str)-> Result<StampyErrorKind,phper::Error>
    {
        match kind {
            "EmptyNameSpace" => Ok(StampyErrorKind::EmptyNameSpace),
            _=>{
                let obj_err = InitializeObjectError::new("Error matching".to_string());
                return Err(phper::Error::InitializeObject(obj_err));
            }
        }
    }

    pub fn get_type_error(&self)->Result<&StampyErrorKind,phper::Error>
    {
        if let Some(kind) = &self.error_type {
            Ok(kind)
        }else {
            let error = Box::new(GlobalHandlerError::Unset);
            Err(phper::Error::Boxed(error))
        }
    }

}

