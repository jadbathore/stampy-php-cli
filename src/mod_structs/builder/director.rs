use crate::mod_traits::builder::class::{BuilderClass,BuilderPropertyClass};

pub struct Director;

impl Director {
    pub fn construct_builder_class(builder: &mut impl BuilderClass,className:&str){
        builder.set_class(className);
        builder.set_methods();
    }

    // pub fn construct_namespacehandler(builder: &mut impl BuilderClass){
    //     builder.set_class("NameSpaceHandler");
    //     builder.set_methods();
    // }
}