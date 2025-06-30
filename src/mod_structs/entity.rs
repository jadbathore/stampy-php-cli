use phper::classes::ClassEntity;


pub struct DialoguerEntity {
    class: ClassEntity<()>
}

impl<'a> DialoguerEntity {
    pub fn new(class:ClassEntity<()>)->Self
    {
        Self { 
            class: class
        }
    }

    pub fn class(&self)-> &ClassEntity<()>
    {
        &self.class
    }
}