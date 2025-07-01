
// use phper::{
//     arrays::ZArray, classes::{ClassEntity,Visibility}, functions::{Argument, MethodEntity}, objects::StateObj, types::{ArgumentTypeHint, Scalar}, values::ZVal
// };
// use dialoguer::{console::Term, theme::{self, ColorfulTheme}, Input, MultiSelect, Password, Select};
// use crate::
//     {
//         mod_enums, 
//         mod_traits::{self, builder::builder_class::BuilderPropertyClass},
//     };
// use mod_enums::{
//     arguments::ArgumentUsage,
//     themes::Themes
// };
// use mod_traits::builder::builder_class::BuilderClass;

// #[derive(Default)]
// pub struct ThemeBuilder<T:'static>
// {
//     class:Option<ClassEntity<T>>,
//     theme:Option<ColorfulTheme>
// }

// impl ThemeBuilder<()> {
//     fn set_style(&mut self,this: &mut StateObj<()>, _: &mut [ZVal])->Result<(), phper::Error>
//     {
//         if let Some(self.theme) = &mut self.theme{
//             this.
//         }
//         self.theme;
//         Ok::<(), phper::Error>(())
//     }
// }


// impl<T> ThemeBuilder<T> {
//     fn set_arguments(argument_usage:ArgumentUsage,method_entity:&mut MethodEntity){
//         match argument_usage {
//             ArgumentUsage::StringWithOptionalTheme => {
                
//             },
//             ArgumentUsage::StringAndListWithOptionalTheme => {
                
//             }
//         }
//     }
// }

// impl BuilderPropertyClass for ThemeBuilder<()> 
// {
//     type OutputType = ClassEntity<()>;

//     fn set_class(&mut self,class_name:&str) { 
//         self.class = Some(ClassEntity::new(class_name));
//         self.theme = Some(ColorfulTheme::default());
//     }

//     fn set_methods(&mut self) {
//         if let Some(class) = &mut self.class {
//             class
//         }
//     }
    
//     fn set_property(&mut self) {
//         if let (Some(theme),Some(class)) = (&mut self.theme,&mut self.class) {
//             class.add_property("themeEntity", Visibility::Private, &theme);
//         }
//     }

//     fn build(self) -> Self::OutputType { 
//         self.class.expect("no class supply")
//     }
// }