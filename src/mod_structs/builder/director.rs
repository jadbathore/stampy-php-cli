use crate::mod_traits::builder::class::BuilderClass;

pub struct Director;

impl Director {
    pub fn construct_dialoguer(builder: &mut impl BuilderClass){
        builder.set_class("Dialoguer");
        builder.set_methods();
    }

    pub fn construct_namespacehandler(builder: &mut impl BuilderClass){
        builder.set_class("NameSpaceHandler");
        builder.set_methods();
    }
}