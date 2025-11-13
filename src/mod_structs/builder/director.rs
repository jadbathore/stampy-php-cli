use crate::mod_traits::builder::class::{BuilderClass,BuilderWrapper};

pub struct Director;

impl Director {
    pub fn construct_builder_class(builder: &mut impl BuilderClass){
        builder.set_class();
        builder.set_methods();
    }

    pub fn construct_wrapper(builder: &mut impl BuilderWrapper){
        builder.set_item();
    }
}