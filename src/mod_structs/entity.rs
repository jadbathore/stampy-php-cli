use console::Term;
use dialoguer::{console, Confirm, Input, MultiSelect, Select, Password};
use phper::{
    arrays::ZArray, 
    classes::{ClassEntity}, 
    values::ZVal,
};
use crossterm::{ExecutableCommand};

pub struct DialoguerEntity {
    class: ClassEntity<()>,
}

impl DialoguerEntity {
    pub fn new(class:ClassEntity<()>)->Self
    {
        Self { class: class }
    }

    pub fn class(&self)-> &ClassEntity<()>
    {
        &self.class
    }

}