use std::{
    error::Error, fmt, path::PathBuf
};

use dialoguer::console::Style;
use phper::values::ZVal;

use crate::{general, paddingPrintln};

#[derive(Debug)]
pub enum GlobalHandlerError {
    Password,
    UnKnownTerminal(String),
    Context(String),
    Unset
}

impl fmt::Display for GlobalHandlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match self {
            GlobalHandlerError::Password => "bad password",
            GlobalHandlerError::Context(string) => &("can't use".to_string() + &string + " in this context"),
            GlobalHandlerError::UnKnownTerminal(string ) => &("Unknown terminal".to_string() + string),
            GlobalHandlerError::Unset => "variable unset"
        };
        f.write_str(description)
    }
}

impl Error for GlobalHandlerError {}

#[derive(Debug)]
pub enum StampyErrorKind {
        EmptyNameSpace,
}


impl StampyErrorKind {
    pub fn match_string(&self)->String
    {
        match self {
            Self::EmptyNameSpace => "EmptyNameSpace".to_string()
        }
    }

    pub fn format_message(&self,arguments:&mut [&ZVal])->Result<(),phper::Error>
    {
        match self {
            StampyErrorKind::EmptyNameSpace => {
                let message:&str = arguments[0].expect_z_str()?.to_str()?;
                let file_arg:&str = arguments[1].expect_z_str()?.to_str()?;
                let mut file_buf = PathBuf::from(file_arg);
                file_buf.pop();
                let red = Style::new().red();
                let green = Style::new().green();
                let format_message:&str = &(String::from("Error: ") + message);
                paddingPrintln!(format_message);

                println!("\n\nYou can't have a namespace with no class attach to it.");
                let path_dir = file_buf.as_os_str().to_string_lossy();
                println!("{path_dir}");
                let a = general::get_entries(file_buf.clone())?;
                for entries in a
                {  
                    let binder = entries.path();
                    let path_cow = binder.as_os_str().to_string_lossy();
                    if file_arg == path_cow {
                        println!(" {}{path_cow}",red.apply_to("├──"));
                    } else {
                        println!(" {}{path_cow}",green.apply_to("├──"));
                    }
                }
            }
        }
        std::process::exit(1);

    }
}
