use phper::{classes::ClassEntry, errors::ThrowObject, values::ZVal};

pub fn leak_value(str:String)->&'static str
{
    Box::leak(str.into_boxed_str())
}

pub fn format_throwable_error(message:&str)-> Result<ThrowObject,phper::Error>
{
    let z_val = ZVal::from(message);
    let std_class = ClassEntry::from_globals("Error")?;
    let zobj = std_class.new_object([z_val])?;
    let mapped_err = ThrowObject::new(zobj)
    .map_err(|e| phper::Error::NotImplementThrowable(e));
    mapped_err
}