use phper::{functions::Argument, modules::Module, php_get_module};

mod hello;
mod dialoguer_class;
mod dialoguer_enum;
pub mod enums;
pub mod structs;



#[php_get_module]
pub fn get_module() -> Module {
    let mut module = Module::new(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );
    module.add_function("say_hello", hello::say_hello).argument(Argument::new("name"));
    module.add_class(dialoguer_class::class_dialoguer());
    module.add_enum(dialoguer_enum::enum_dialoguer());
    module
}
