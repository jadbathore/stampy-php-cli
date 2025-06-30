use phper::{modules::Module, php_get_module};
// use dialoguer::Password;

mod dialoguer_class;
mod dialoguer_enum;
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
    // let mut password_instance:Password  = Password::new();
    // module.add_class(dialoguer_class::class_dialoguer());
    module.add_enum(dialoguer_enum::enum_dialoguer());
    module
}
