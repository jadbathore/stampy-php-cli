use phper::{modules::Module, php_get_module};

use crate::{
    mod_structs::{builder::{class::DialoguerBuilder, director::Director, namespacehandler::NamespaceHandler}, namespace_buf::ClassesInNamespace}, 
    mod_traits::builder::class::{BuilderClass, BuilderPropertyClass}
};
// use dialoguer::Password;

pub mod mod_enums;
pub mod mod_structs;
pub mod mod_traits;



#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    let mut dialoguer_builder:DialoguerBuilder<()> = DialoguerBuilder::default();
    Director::construct_dialoguer(&mut dialoguer_builder);
    let dialoguer_class = dialoguer_builder.build();
    module.add_class(dialoguer_class);

    let mut namespacehandler_builder:NamespaceHandler<ClassesInNamespace> = NamespaceHandler::default();
    Director::construct_namespacehandler(&mut namespacehandler_builder);
    let namespacehandler_class = namespacehandler_builder.build();
    module.add_class(namespacehandler_class);

    module
}
