use std::{ffi::{OsStr}, fs::{self, DirEntry}, io, path::PathBuf};
use std::borrow::Cow;
use phper::arrays::{ZArray};

#[derive(Default,Clone)]
pub struct ClassesInNamespace<'a>
{
    items:Cow<'a,[&'static str]>,
    path:PathBuf
}

impl  ClassesInNamespace<'_> {

    fn resolve_callback(index:usize,entry:&mut DirEntry,self_clone:&mut ClassesInNamespace<'_>,classes_reciever:&mut ZArray)->Result<(), io::Error>
    {
        let mut sub_path:PathBuf = entry.path();
        sub_path.set_extension("");

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

    pub fn resolver<'a>(&mut self)->Result<ZArray,io::Error>
    {
        let a = self.resolve(Self::resolve_callback)?;
        Ok(a)
    }

    

    fn resolve<A>(&mut self,builder:A) -> Result<ZArray,io::Error>
    where 
        A:for<'a> Fn(usize,&mut DirEntry,&mut ClassesInNamespace<'a>,&mut ZArray) -> Result<(),io::Error>,
    {
        let mut z_array = ZArray::new();

        for i in 0..self.items.len() {
            let mut path_buf:PathBuf = self.path.clone();
            path_buf.push(self.items[i]);
            if path_buf.exists() {
                let rests = &self.items[i+1..];
                path_buf.extend(rests);
                let mut iterable = self.dir_entry(&path_buf)?;
                for (i,value) in iterable.iter_mut().enumerate() {
                    builder(i,value,&mut self.clone(),&mut z_array)?;
                } 
                break;
            }
        }
        if z_array.is_empty() {
            let mut iterable = self.dir_entry(&self.path.clone())?;
            for (i,value) in iterable.iter_mut().enumerate() {
                builder(i,value,&mut self.clone(),&mut z_array)?;
            } 
        }
        Ok(z_array)
    }

    fn dir_entry<'a>(&mut self,path_buf:& PathBuf) -> Result<Vec<DirEntry>,io::Error>
    {
        fs::read_dir(path_buf)?.collect::<Result<Vec<DirEntry>, io::Error>>()
    }
}
