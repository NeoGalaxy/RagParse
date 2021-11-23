use std::any::type_name;
use std::str::FromStr;

use crate::parser::ArgParseResult;
use ArgParseResult::*;

pub type ArgParser<'a,T> = &'a dyn Fn(&str) -> ArgParseResult<T>;
pub struct Arg<'a, T> {
	name: &'a str,
	descr: &'a str,
	parser: ArgParser<'a, T>,
	required: bool,
	value: Option<Box<T>>
}

pub trait ArgTrait {
	fn get_name(&self) -> &str;
	fn get_descr(&self) -> &str;
	fn parse(&mut self, arg: &str) -> ArgParseResult<()>;
	fn is_required(&self) -> bool;
	fn type_name(&self) -> &str;
	fn reset(&mut self);
}

impl<T: FromStr> Arg<'_, T> {
	pub fn new<'a>(name: &'a str, descr: &'a str) -> Arg<'a, T>
	{
		Arg {
			name,
			descr,
			parser: &crate::parser::any,
			required: true,
			value: None
		}
	}
}

impl<T> Arg<'_, T> {
	pub fn take(&mut self) -> Option<T> {
		let val = self.value.take();
		return match val {
			None => None,
			Some(val) => Some(*val)
		}
	}
}

impl<'life, T: Clone> Arg<'life, T> {
	pub fn set_parser(mut self, parser: ArgParser<'life, T>, default: Box<T>) -> Self {
		self.parser = parser;
		self.value = Some(default);
		self
	}
	pub fn optionnal(mut self) -> Self {
		self.required = false;
		self
	}
	pub fn get(&self) -> T {
		self.value.as_ref().unwrap().as_ref().clone()
	}
	pub fn get_opt(&self) -> Option<T> {
		match &self.value {
			Some(val) => Some(*val.clone()),
			None => None,
		}
	}
}

impl<T> ArgTrait for Arg<'_, T> {
	fn reset(&mut self) {
		self.value = None;
	}
	fn get_name(&self) -> &str {
		&self.name
	}
	fn get_descr(&self) -> &str {
		&self.descr
	}
	fn parse(&mut self, arg: &str) -> ArgParseResult<()> {
		match (self.parser)(arg) {
			ParseOk(value) => {
				self.value = Some(Box::new(value));
				return ParseOk(());
			},
			DispHelp => DispHelp,
			TypeError(errmsg) => return TypeError(errmsg),
		}
	}
	fn is_required(&self) -> bool {
		self.required
	}
	fn type_name(&self) -> &str {
		type_name::<T>()
	}
}

