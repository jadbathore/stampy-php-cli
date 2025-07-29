
#[macro_export]
macro_rules! passthrough {

    (init $struct:ident<$type_arg:ty> as $var:ident) => {

        struct $struct {
            item: Option<$type_arg>,
        }

        impl $struct {
            fn new()->Self
            {
                $struct{ item:None }
            }

            fn set_item(&mut self,argument:$type_arg){
                self.item = Some(argument);
            }

            fn get_item(self)-> Option<$type_arg>
            {
                self.item
            }
        }

        let mut $var:$struct = $struct::new(); 
    };



    (init $struct:ident<$a:lifetime, $type_arg:ty> as $var:ident) => {

        struct $struct<$a> {
            item: Option<std::borrow::Cow<$a,$type_arg>>,
        }

        impl<$a> $struct<$a> {
            fn new()->Self
            {
                $struct{item:None}
            }

            fn set_item(&mut self,argument:$type_arg){
                self.item = Some(std::borrow::Cow::Owned(argument));
            }

            fn get_item(self)-> Option<std::borrow::Cow<$a,$type_arg>>
            {
                self.item
            }
        }

        let mut $var:$struct<$a> = $struct::new(); 
    };

    (init $struct:ident<$a:lifetime, &$type_arg:ty> as $var:ident) => {

        struct $struct<$a> {
            item: Option< std::borrow::Cow<$a,$type_arg>>,
        }

        impl<$a> $struct<$a,$type_arg> {
            fn new()->Self
            {
                $struct{item:None}
            }

            fn set_item(&mut self,argument:$type_arg){
                self.item = Some(std::borrow::Cow::Borrow(argument));
            }

            fn get_item(&mut self)-> Option<std::borrow::Cow<$a,$type_arg>>
            {
                self.item
            }
        }

        let mut $var:$struct<$a> = $struct::new(); 
    };

    (set $var:ident = $val:expr) => {
        $var.set_item($val);
    };

    (get $var:ident) => {
        $var.get_item()
    };


    // règle finale vide (terminaison)
    () => {};
}
