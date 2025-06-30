
use std::collections::HashMap;

 #[derive(Eq, Hash, PartialEq)]
enum ArgumentUsage {
    StringWithOptionalTheme,
    StringAndListWithOptionalTheme
}

fn main()->Result<(),Box<dyn std::error::Error>>
{
    let mut vec:Vec<(ArgumentUsage, String)> = Vec::new();
    vec.push((ArgumentUsage::StringAndListWithOptionalTheme,String::from("a")));
    vec.push((ArgumentUsage::StringAndListWithOptionalTheme,String::from("b")));
    vec.push((ArgumentUsage::StringWithOptionalTheme,String::from("c")));
    vec.push((ArgumentUsage::StringAndListWithOptionalTheme,String::from("d")));
    vec.push((ArgumentUsage::StringWithOptionalTheme,String::from("e")));
    vec.push((ArgumentUsage::StringAndListWithOptionalTheme,String::from("f")));

    for (a,b) in vec.iter(){
        println!("{}",b);
        match a {
            ArgumentUsage::StringAndListWithOptionalTheme => {
                println!("StringWithOptionalTheme {b:?}")
            },
            ArgumentUsage::StringWithOptionalTheme => {
                println!("StringAndListWithOptionalTheme {b:?}")
            }
        }
    }

    Ok(())
}
