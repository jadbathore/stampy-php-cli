pub mod mod_enums;
pub mod mod_structs;
pub mod mod_traits;
pub mod general;
pub mod function;
pub mod general_macro;
use phper::{classes::ClassEntity, modules::Module, php_get_module};
use crate::{
    mod_structs::{
        builder::{
            class::DialoguerBuilder,
            console::ConsoleDialoguer, 
            director::Director, indicatif::ProgressBarBuilder, 
            namespacehandler::NamespaceHandler, stampyerrors::StampyExceptionBuilder
        }, 
    error_handler::StampyKindHandler, namespace_buf::ClassesInNamespace}, 
    mod_traits::builder::class::BuilderClass
};

pub const TTY:&str = "dev/tty";
pub const STDOUT:&str = ">";
pub const STDERR:&str = "2>";
pub const STDIN:&str = "<";

#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );
    
    let mut dialoguer_builder:DialoguerBuilder<()> = DialoguerBuilder::default();
    Director::construct_builder_class(&mut dialoguer_builder);
    let dialoguer_class = dialoguer_builder.build();
    module.add_class(dialoguer_class);

    let mut namespacehandler_builder:NamespaceHandler<ClassesInNamespace> = NamespaceHandler::default();
    Director::construct_builder_class(&mut namespacehandler_builder);
    let namespacehandler_class = namespacehandler_builder.build();
    module.add_class(namespacehandler_class);

    let mut namespacehandler_builder:ProgressBarBuilder = ProgressBarBuilder::default();
    Director::construct_builder_class(&mut namespacehandler_builder);
    let namespacehandler_class = namespacehandler_builder.build();
    module.add_class(namespacehandler_class);

    let mut stampy_exception_builder:StampyExceptionBuilder<StampyKindHandler> = StampyExceptionBuilder::default();
    Director::construct_builder_class(&mut stampy_exception_builder);
    let stampy_exception_class = stampy_exception_builder.build();
    module.add_class(stampy_exception_class);
    
    let mut stampy_error_kind = ClassEntity::new("StampyErrorKind");
    stampy_error_kind.add_static_property("EmptyNameSpace", phper::classes::Visibility::Private, ());
    module.add_class(stampy_error_kind);

    let mut console_dialoguer_builder:ConsoleDialoguer = ConsoleDialoguer::default();
    Director::construct_builder_class(&mut console_dialoguer_builder);
    let dialoguer_console = console_dialoguer_builder.build();
    module.add_class(dialoguer_console);
    module.add_constant("STDOUT_KEY", STDOUT);
    module.add_constant("STDERR_KEY", STDERR);
    module.add_constant("STDIN_KEY", STDERR);
    module.add_function("padding", function::padding);
    module
}
