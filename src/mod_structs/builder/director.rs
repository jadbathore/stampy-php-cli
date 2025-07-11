use crate::mod_traits::builder::class::{BuilderClass,BuilderPropertyClass};

pub struct Director;

impl Director {
    pub fn construct_dialoguer(builder: &mut impl BuilderClass){
        builder.set_class("Dialoguer");
        builder.set_methods();
    }

    pub fn construct_namespacehandler(builder: &mut impl BuilderPropertyClass){
        builder.set_class("NameSpaceHandler");
        builder.set_property();
        builder.set_methods();
    }
}