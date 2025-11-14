#[cfg(not(feature = "docker"))]
use std::borrow::Cow;
use std::{fs::{self, DirEntry}, io, path::PathBuf};

use phper::{ classes::ClassEntry, errors::ThrowObject, objects::ZObject, values::ZVal};

use crate::mod_enums::errors::class_error::StampyErrorKind;

pub fn leak_value(str:String)-> &'static str
{
    Box::leak(str.into_boxed_str())
}

pub fn format_throwable_error(message:&str)-> Result<ThrowObject,phper::Error>
{
    let z_val = ZVal::from(message);
    let error_class = ClassEntry::from_globals("Error")?;
    let zobj = error_class.new_object([z_val])?;
    ThrowObject::new(zobj)
    .map_err(|e| phper::Error::NotImplementThrowable(e))
}

pub fn format_throwable_exception(path:PathBuf,error:io::Error,stampy_error_kind:StampyErrorKind)-> Result<ThrowObject,phper::Error>
{
    let error_class = ClassEntry::from_globals("StampyException")?;
    let message = ZVal::from(error.to_string());
    let file_from = ZVal::from(path.as_path().to_str());
    let type_from = ZVal::from(stampy_error_kind.match_string());
    let zobj:ZObject = error_class.new_object([message,file_from,type_from])?;
    ThrowObject::new(zobj)
    .map_err(|e| phper::Error::NotImplementThrowable(e))
}

pub fn get_entries(path:PathBuf)-> Result<Vec<DirEntry>,io::Error>
{
    Ok(fs::read_dir(path)?.collect::<Result<Vec<DirEntry>,io::Error>>()?)
}

#[cfg(feature = "docker")]
pub fn get_tty<'a>()->Result<Cow<'a,str>,io::Error>
{
    let tty_path = std::fs::read_link("/proc/self/fd/0")?;
    if let Some(linkfile) = tty_path.as_os_str().to_str() {
        Ok(Cow::from(String::from(linkfile)))
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidFilename, "bad file name"))
    }
}

#[cfg(not(feature = "docker"))]
pub fn get_tty<'a>()->Cow<'a,str>
{
    Cow::from(String::from("/dev/tty"))
}

