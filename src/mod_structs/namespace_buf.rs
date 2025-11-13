


use std::{ffi::{OsStr, OsString}, fs::{ DirEntry}, io::{self, Error}, path::PathBuf};
use std::borrow::Cow;
use phper::arrays::{ZArray};

use crate::{general, passthrough};

#[derive(Default,Clone)]
pub struct ClassesInNamespace<'a>
{
    items:Cow<'a,[&'static str]>,
    path:PathBuf
}

impl ClassesInNamespace<'_> 
{

    fn resolve_callback(index:usize,entry:&mut DirEntry,self_clone:&mut ClassesInNamespace<'_>,classes_reciever:&mut ZArray)->Result<(),(PathBuf,io::Error)>
    {

        let mut sub_path:PathBuf = entry.path();
        sub_path.set_extension("");
        if sub_path.is_dir() {
            let err = io::Error::new(io::ErrorKind::IsADirectory, "empty namespace");
            return Err((sub_path,err));
        }
        if let Some(file) =  sub_path.file_name()
        {
            let namespace  = Self::format(file,self_clone.get_namespace()); 
            if let Some (class) = namespace {
                classes_reciever.insert(index as u64, class);
            }
        }
        Ok(()) 
    }

    fn format(os_str:&OsStr,str:String)-> Option<String>
    {
        if let Some(item) = os_str.to_str(){
            let a = str + "\\" + item;
            Some(a)
        } else {
            None
        }
    }
}

impl<'b>  ClassesInNamespace<'b> 
    where 'static:'b
{

    pub fn new()-> ClassesInNamespace<'b>
    {
        ClassesInNamespace { 
            items: Cow::from(Vec::new()),
            path:PathBuf::new()
        }
    }

    pub fn from(slice:&'b[&'static str],path:PathBuf)-> ClassesInNamespace<'b>
    {
        ClassesInNamespace { 
            items: Cow::from(slice),
            path:path
        }
    }

    pub fn pop(&mut self)
    {
        self.items.to_mut().pop();
    }

    pub fn get_namespace(&self)-> String
    {
        self.items.join("\\")
    }

    pub fn push_to_path(&mut self,item:&'static str){
        self.path.push(item);
    }

    pub fn push_to_namespace(&mut self,item:&'static str)
    { 
        self.items.to_mut().push(item.as_ref());
    }

    pub fn extend_to_namespace(&mut self,item:&[&'static str])
    { 
        self.items.to_mut().extend_from_slice(item);
    }

    pub fn resolver<'a>(&mut self)->Result<ZArray,(PathBuf,io::Error)>
    {
        Ok(self.clone().resolve(Self::resolve_callback)?)
    }


    fn get_starting_path(&mut self)->Option<(usize,PathBuf)>
    {
        passthrough!(init Item<(usize,PathBuf)> as path_through);
        for i in 0..self.items.len() {
            let mut path_buf:PathBuf = self.path.clone();
            path_buf.push(self.items[i]);
            if path_buf.exists() {
                passthrough!(set path_through = (i+1,path_buf));
                break;
            }
        }
        passthrough!(get path_through)
    }


    fn resolve<A>(mut self,builder:A) -> Result<ZArray,(PathBuf,io::Error)>
    where 
        A:for<'a> Fn(usize,&mut DirEntry,&mut ClassesInNamespace<'a>,&mut ZArray) -> Result<(),(PathBuf,io::Error)>,
    {
        let mut z_array = ZArray::new();
        if let Some((start_slice,mut path_buf)) = self.get_starting_path() {
            let rests = &self.items[start_slice..];
            path_buf.extend(rests);
            let mut vec_entries = general::get_entries(path_buf.clone()).map_err(|err|(path_buf,err))?;
            self.populate_z_array(&mut vec_entries,&mut  z_array, builder)?;
        } else {
            let result = general::get_entries(self.path.clone());
            if let Err(err) = result  {
                return Err((self.path,err))
            }
            let mut vec_entries = result.unwrap();
            self.populate_z_array(&mut vec_entries,&mut  z_array, builder)?;
        }
        Ok(z_array)
    }

    pub fn try_push(&mut self,namepace_slice:&OsString)-> Result<bool,io::Error>
    {
        if let Some(last_namespace) = self.items.last() {
            let mut clone_path = self.path.clone();
            clone_path.push(last_namespace);
            let vec_entries: Vec<OsString> = general::get_entries(clone_path)?.into_iter()
            .map(|x|{
                x.file_name()
            }).collect();
            Ok(vec_entries.contains(&namepace_slice))
        } else {
            Err(Error::new(io::ErrorKind::AddrInUse, "wrong path format"))?
        }
    }

    fn populate_z_array<A>(&mut self,vec_entries:&mut Vec<DirEntry>,z_array:&mut ZArray,builder:A)-> Result<(),(PathBuf,io::Error)>
    where 
        A:for<'a> Fn(usize,&mut DirEntry,&mut ClassesInNamespace<'a>,&mut ZArray) -> Result<(),(PathBuf,io::Error)>,
    {
        for (i,value) in vec_entries.iter_mut().enumerate() {
            builder(i, value,&mut self.clone(), z_array)?;
        }
        Ok(())
    }
}
