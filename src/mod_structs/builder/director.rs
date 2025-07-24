use crate::mod_traits::builder::class::{BuilderClass,BuilderWrapper};

pub struct Director;

impl Director {
    pub fn construct_builder_class(builder: &mut impl BuilderClass,class_name:&str){
        builder.set_class(class_name);
        builder.set_methods();
    }

    pub fn construct_wrapper(builder: &mut impl BuilderWrapper){
        builder.set_item();
    }
}