
use std::fs::File;

use phper::errors::InitializeObjectError;

use crate::{
    mod_structs::{builder::console::{ConsoleDialoguer, StdStampy}},
    mod_traits::builder::class::BuilderClass
};


#[derive(Default)]
pub struct TerminalHandler
{
    inner:Option<StdStampy>
}




impl TerminalHandler {
    pub fn get_term(&mut self)->Result<&mut StdStampy,phper::Error>
    {
        if let Some(terminal) = &mut self.inner {
            Ok(terminal)
        } else {
            let error = InitializeObjectError::new(<ConsoleDialoguer as BuilderClass>::CLASS_NAME.to_string());
            Err(phper::Error::InitializeObject(error))
        }
    }
}

pub trait TerminalTarget
{
    fn set_terminal_output(&mut self,term:StdStampy);
}

pub trait TerminalReadWrite
{
    fn set_terminal_output(&mut self,read:File,write:StdStampy)->Result<(),phper::Error>;
}

impl TerminalTarget for TerminalHandler {
    fn set_terminal_output(&mut self,term:StdStampy)
    {
        self.inner = Some(term);
    }
}

impl TerminalReadWrite for TerminalHandler {
    fn set_terminal_output(&mut self,read:File,write:StdStampy)->Result<(),phper::Error>
    {
        self.inner = Some(StdStampy::ReadWritePair(read,Box::new(write)));
        Ok(())
    }
}
