// use std::{borrow::Cow, path::PathBuf};


// trait PassthroughItem<T> {
//     fn set_item(&mut self,argument:T);
//     fn get_item(self)-> Option<T>;
// }



// macro_rules! test_create_class {
//     (init $struct:ident) => {
//         struct $struct<T> {
//             item: Option<T>,
//         }

//         impl<T> $struct<T> {
//             fn new()->Self
//             {
//                 $struct{ item:None }
//             }
//         }
//     };

//     (init $struct:ident<$a:lifetime>) => {
//         struct $struct<$a,T:Clone> {
//             item: Option<Cow<$a,T>>,
//         }

//         impl<'a,T:Clone> $struct<'a,T> {
//             fn new()->Self
//             {
//                 $struct{ item:None }
//             }
//         }
//     };

//     ($struct:ident<$type_arg:ty> as $var:ident) => {
//         test_create_class!(init $struct);

//         impl $struct<$type_arg> {
//             fn get_item(self)-> Option<$type_arg>
//             {
//                 self.item
//             }
//         }

//         let $var:$struct<$type_arg> = $struct::new();
//     };

//     ($struct:ident<&$a:lifetime $type_arg:ty> as $var:ident) => {
//         test_create_class!(init $struct<$a>);

//         impl<$a> $struct<$a,$type_arg> {
//             fn get_item(self)-> Option<Cow<$a,$type_arg>>
//             {
//                 self.item
//             }
//         }

//         let $var:$struct<$type_arg> = $struct::new();
//     };



//     (get $var:ident) => {
//         $var.get_item()
//     };
// }

// fn test<'a>(){
//     test_create_class!(Item<'a,PathBuf> as test);
//     test_create_class!(get test);
// }



fn main() {

}


// fn test<'a>()-> Option<PathBuf>
// {
//     passthrough!(init Item<PathBuf> as item);
//     for i in 0..5 {
//         if i == 4 {
//             let mut a = PathBuf::new();
//             a.push("hello");
//             passthrough!(set item = a);
//         }
//     };
//     passthrough!(get item)
// }

// macro_rules! yeilder {
    
//     (for ($($x:expr),*)$block:stmt; $($rest:tt)*) => {
//         $(
//             $block
//         )*;
//         yeilder! { $($rest)* }
//     };

//     (set $var:ident = $val:expr; $($rest:tt)* ) => {
//         let $var = $val;
//         yeilder! { $($rest)* }
//     };

//     // règle pour `print ident;`
//     (print $var:ident; $($rest:tt)* ) => {
//         println!("{}", $var);
//         yeilder! { $($rest)* }
//     };

//     // règle finale vide (terminaison)
//     () => {};
// }