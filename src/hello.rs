use phper::{values::ZVal,echo};

pub fn say_hello(arguments: &mut [ZVal]) -> phper::Result<()> {

    let name = arguments[0].expect_z_str()?.to_str()?;

    echo!("Hello, {}!\n", name);

    Ok(())
}