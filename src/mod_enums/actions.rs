use dialoguer::{Confirm, Input, MultiSelect, Select,Password};



pub enum Actions<T> {
    Input(Input<'static,T>),
    Confirm(Confirm<'static>),
    MultiSelect(MultiSelect<'static>),
    Select(Select<'static>),
    Password(Password<'static>)
}

