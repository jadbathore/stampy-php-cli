use crate::mod_traits::builder::builder_class::BuilderClass;

pub struct Director;

impl Director {
    pub fn construct_dialoguer(builder: &mut impl BuilderClass){
        builder.set_class("Dialoguer");
        builder.set_methods();
    }
}