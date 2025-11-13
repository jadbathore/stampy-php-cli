use std::{ fs::{File, OpenOptions}, io::{self, Cursor, Write}};

use phper::{
    classes::{ClassEntity, Visibility}, 
    errors::ArgumentCountError, 
    objects::StateObj, values::ZVal
};




use crate::{
        STDERR, STDIN, STDOUT, 
        mod_enums::errors::class_error::GlobalHandlerError, 
        mod_structs::terminal::{TerminalHandler, TerminalReadWrite, TerminalTarget}, 
        mod_traits::builder::class::BuilderClass
    };


#[derive(Default)]
pub struct ConsoleDialoguer
{
    class:Option<ClassEntity<TerminalHandler>>,
}

pub enum StdStampy {
    StdOut(File),
    StdErr(Cursor<Vec<u8>>),
    ReadWritePair(File,Box<StdStampy>)
}

impl StdStampy {
    fn writeto(&mut self,input:&str)->Result<(),io::Error>
    {
        match self {
            Self::StdErr(cursor)=>{
                cursor.write(input.as_bytes())?;
            },
            Self::StdOut(tty)=>{
                tty.write(input.as_bytes())?;
            },
            Self::ReadWritePair(_, tty_output) => {
                tty_output.writeto(input)?;
            }
        };
        Ok(())
    }

    fn flush(&mut self)->Result<(),io::Error>
    {
        match self {
            Self::StdErr(cursor)=>{
                let mut tty = OpenOptions::new().write(true).open("/dev/tty")?;
                tty.write(cursor.get_mut())?;
                std::process::exit(1);
            },
            Self::StdOut(_) => {
                Err(io::Error::new(io::ErrorKind::Other, "the Stdout does Not Flush"))
            },
            Self::ReadWritePair(_, tty_output) => {
                StdStampy::flush(tty_output)?;
                Ok(())
            }
        }
    }
}
fn term_target_value(z_value:&ZVal)->Result<StdStampy,phper::Error>
{
    let term_type_input = z_value.expect_z_str()?.to_str()?;
    
    match term_type_input {
        STDOUT => {
            let tty = OpenOptions::new().write(true).open("/dev/tty")?;
            Ok(StdStampy::StdOut(tty))
        },
        STDERR => {
            let capacity_vec:Vec<u8> = Vec::with_capacity(8192);
            let cursor = Cursor::new(capacity_vec);
            Ok(StdStampy::StdErr(cursor))
        },
        value => {
            let error = Box::new(GlobalHandlerError::Context(value.to_string()));
            return Err(phper::Error::Boxed(error));
        }
    }
}




impl ConsoleDialoguer
{

    fn terminal_use(this:&mut StateObj<TerminalHandler>,arguments:&mut [ZVal])->Result<(),phper::Error>
    {
        let mut arg_list = arguments.iter();
        let arguments_expected  = (arg_list.next(),arg_list.next(),arg_list.next());
        let terminal = this.as_mut_state();
        match arguments_expected {
            (Some(terminal1),None,None) => {
                let std = term_target_value(terminal1)?;
                TerminalTarget::set_terminal_output(terminal,std);
                // write!(,"test");
                
                Ok(())
            },
            (Some(file),Some(tty),None) => {
                let mut stdin_input   = file.expect_z_str()?.to_str()?;
                let std = term_target_value(tty)?;
                if stdin_input == STDIN {
                    stdin_input = "/dev/tty";
                } 
                let stdin = OpenOptions::new().read(true).open(stdin_input)?; 
                // let std2 = term_target_value(terminal2)?;
                TerminalReadWrite::set_terminal_output(terminal,stdin,std)?;
                Ok(())
            }
            _ => {
                return Err(phper::Error::ArgumentCount(ArgumentCountError::new(String::from("push"), 1, arguments.iter().len())));
            }
        }
    }
    fn write(this:&mut StateObj<TerminalHandler>,arguments:&mut [ZVal])->Result<(),phper::Error>
    {
        let binder = this.as_mut_state().get_term()?;
        let mut arg_list = arguments.iter();
        let arguments_expected  = (arg_list.next(),arg_list.next());
        if let (Some(input),None) = arguments_expected {
            let input_str = input.expect_z_str()?.to_str()?;
            binder.writeto(input_str)?;
        }
        Ok(())
    }

    fn flush(this:&mut StateObj<TerminalHandler>,_:&mut [ZVal])->Result<(),phper::Error>
    {
        let binder = this.as_mut_state().get_term()?;
        binder.flush()?;
        Ok(())
    }
    

}


impl BuilderClass for ConsoleDialoguer {
    type OutputType = ClassEntity<TerminalHandler>;
    const CLASS_NAME:&str = "ConsoleTTY";

    fn set_class(&mut self)
    {  
        self.class = Some(ClassEntity::new_with_state_constructor(Self::CLASS_NAME, TerminalHandler::default));
    }

    fn set_methods(&mut self) {
        if let Some(class) = &mut self.class {
            class.add_method("__construct", Visibility::Public,Self::terminal_use);
            class.add_method("write", Visibility::Public, Self::write);
            class.add_method("flush", Visibility::Public, Self::flush);
        }   
    }

    fn build(self) -> Self::OutputType { 
        self.class.expect("no class supply")
    }
}
