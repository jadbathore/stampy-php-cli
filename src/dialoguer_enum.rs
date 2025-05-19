// use console::Term;
// use dialoguer::theme::{ColorfulTheme, SimpleTheme};
use phper::{
    // alloc::EBox, classes::Visibility,
 enums::EnumEntity,
//  objects::ZObj, values::ZVal
};


pub fn enum_dialoguer()-> EnumEntity<String>
{ 
    let mut enums:EnumEntity<String> = EnumEntity::new("Theme");
    enums.add_case("ColorfulTheme",String::from("ColorfulTheme"));
    enums.add_case("SimpleTheme", String::from("SimpleTheme"));
    enums
}



// fn method_get_theme_by_name<'a>(arguments:&'a mut [ZVal])->Result<Option<ColorfulTheme>, phper::Error>
// {

//     let e_box:Option<ColorfulTheme> = Some(ColorfulTheme::default());
//     let e_box = Box::new(e_box);
//     Ok(e_box)
// }
// #[derive(Debug)]
// struct ObjectTheme<'a,T> {
//     theme:T,
//     z_object:&'a ZObj
// }
// impl<'a,T> ObjectTheme<'a,T> {
//     fn new(theme:T,z_obj:&'a ZObj)-> Self
//     {
//         ObjectTheme {
//             theme:theme,
//             z_object: z_obj
//         }
//     }
// }


