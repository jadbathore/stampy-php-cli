// use phper::{modules::Module, php_get_module};

// pub mod mod_enums;
// pub mod mod_structs;
// pub mod mod_traits;
// pub mod general;
// pub mod general_macro;
// use crate::{
//     mod_structs::{builder::{class::DialoguerBuilder, director::Director, indicatif::ProgressBarBuilder, namespacehandler::NamespaceHandler}, namespace_buf::ClassesInNamespace}, 
//     mod_traits::builder::class::BuilderClass
// };

// #[php_get_module]
// pub fn get_module() -> Module {
//     let mut module = Module::new(
//         env!("CARGO_PKG_NAME"),
//         env!("CARGO_PKG_VERSION"),
//         env!("CARGO_PKG_AUTHORS"),
//     );

//     let mut dialoguer_builder:DialoguerBuilder<()> = DialoguerBuilder::default();
//     Director::construct_builder_class(&mut dialoguer_builder,"Dialoguer");
//     let dialoguer_class = dialoguer_builder.build();
//     module.add_class(dialoguer_class);

//     let mut namespacehandler_builder:NamespaceHandler<ClassesInNamespace> = NamespaceHandler::default();
//     Director::construct_builder_class(&mut namespacehandler_builder,"NamespaceHandler");
//     let namespacehandler_class = namespacehandler_builder.build();
//     module.add_class(namespacehandler_class);

//     let mut namespacehandler_builder:ProgressBarBuilder = ProgressBarBuilder::default();
//     Director::construct_builder_class(&mut namespacehandler_builder,"Indicatif");
//     let namespacehandler_class = namespacehandler_builder.build();
//     module.add_class(namespacehandler_class);
//     module
// }
