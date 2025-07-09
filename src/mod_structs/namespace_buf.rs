use std::ffi::{OsStr, OsString};


pub struct NameSpaceBuf {
    inner: OsString,
}

impl NameSpaceBuf {

    pub fn new()-> NameSpaceBuf
    {
        Self {
            inner : OsString::new(),
        }
    }

    pub fn from(namespace:&str)-> NameSpaceBuf
    {
        let mut inner = OsString::new();
        inner.push(namespace);
        Self {
            inner : inner ,
        }
    }

    pub fn push<T>(&mut self,namespace:Option<T>) 
    where 
        T:AsRef<OsStr>
    {
        if let Some(namespace) = namespace {
            self.inner.push("\\");
            self.inner.push(namespace.as_ref());
        }
    }

    pub fn get_namespace(self)->OsString
    {
        self.inner
    }
}



// impl<'b> BuilderNamespaceBuf for NameSpaceBuf<'b> 
// {
//     type OutputType = NameSpaceBuf<'b>;

//     fn set_inner(&mut self) {
        
//     }
    
//     fn set_items(&mut self,namespace:&str) {
//         let a= 
//     }

//     fn build(self) -> Self::OutputType { 
//         self.class.expect("no class supply")
//     }
// }