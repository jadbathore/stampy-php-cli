pub fn class_dialoguer(/*password_state:&mut Password*/)->ClassEntity<()>
{ 
    let mut class:ClassEntity<()> = ClassEntity::new("Theme");
    // class.add_static_property("password_state", Visibility::Private, password_state);
    method_string_plus_theme(
        class.add_static_method("Confirm",Visibility::Public,method_confirm)
    );
    method_string_plus_theme(
        class.add_static_method("input",Visibility::Public,method_input)
    );
    
    class.add_static_method("input",Visibility::Public,method_input);
    class.add_static_method("select",Visibility::Public,method_select);
    class.add_static_method("MultiSelect",Visibility::Public,method_multi_select);
    class.add_static_method("test",Visibility::Public,method_test);
    class.add_static_method("password",Visibility::Public,method_password);
    class
}