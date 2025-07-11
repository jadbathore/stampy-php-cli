use std::{ffi::{OsStr, OsString}, fs, io, path::PathBuf};
use std::borrow::Cow;

use phper::arrays::ZArray;

pub struct ClassesInNamespace<'a> 
{
    items:Cow<'a,[&'a OsStr]>,
    inner:OsString
}

impl<'a> ClassesInNamespace<'a> {

    pub fn new()-> ClassesInNamespace<'a>
    {
        ClassesInNamespace { 
            items: Cow::from(Vec::new()),
            inner: OsString::new()
        }
    }

    pub fn from(slice:&'a[&'a OsStr])-> ClassesInNamespace<'a>
    {
        ClassesInNamespace { 
            items: Cow::from(slice),
            inner: OsString::from(slice.join(OsStr::new("\\")))
        }
    }

    pub fn get_namespace(self)->OsString
    {
        self.items.join(OsStr::new("\\"))
    }

    pub fn get_class(self)->Option<&'a OsStr>
    {
        self.items.last().cloned()
    }

    pub fn push(&mut self,item:&'a OsStr){ 
        self.items.to_mut().push(item);
        self.inner.push(item);
    }


    pub fn resolver(self,path:PathBuf,classes_reciever:&mut ZArray)->Result<(),io::Error>
    {
        for i in 0..self.items.len(){
            let mut path_buf:PathBuf = path.clone();
            path_buf.push(self.items[i]);
            if path_buf.exists() {
                let rests = &self.items[i+1..];
                for rest in rests {
                    path_buf.push(rest);
                } 
                
                for (key,entry) in fs::read_dir(path_buf)?.into_iter().enumerate() {
                    let dir = entry?;
                    let mut path = dir.path();
                    path.set_extension("");
                    if let Some(file) = path.file_name() {
                        let mut test = ClassesInNamespace::new();
                        test.push(&self.inner);
                        test.push(file);
                        classes_reciever.insert(key as u64,test.get_namespace().to_str());
                    }
                }
                break;
            }
        }
        Ok(())
    }
}
