use core::any::Any;


pub trait ArgType: Any {
	fn name() -> &'static str; /*{
		std::any::type_name::<T>()
	}*/
}

pub fn type_name<T>() -> &'static str
	where T: ArgType
{
	T::name()
}

pub fn any<T>(arg: &str) -> Result<Box<T>, String>
	where T: ArgType + std::str::FromStr
{
	match arg.parse() {
		Ok(value) => Ok(Box::new(value)),
		Err(_) => Err(format!("Can't parse argument into {}.", type_name::<T>())),
	}
}

/*impl<T> ArgType for T where T: Any {
	fn name() -> &'static str {
		std::any::type_name::<T>()
	}
}*/

impl ArgType for i16 {
	fn name() -> &'static str {
		"int"
	}
}
impl ArgType for i32 {
	fn name() -> &'static str {
		"int"
	}
}

impl ArgType for String {
	fn name() -> &'static str {
		"string"
	}
}
