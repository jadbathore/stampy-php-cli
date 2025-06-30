use dialoguer::theme::{ColorfulTheme, SimpleTheme};


#[derive(Copy,Clone)]
pub enum Themes<'a> {
    SimpleTheme(&'a SimpleTheme),
    ColorfulTheme(&'a ColorfulTheme)
}