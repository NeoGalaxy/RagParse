use std::any::type_name;
use std::str::FromStr;

use crate::parser::ArgParseResult;
use ArgParseResult::*;

pub type OptParser<'a,T> = &'a dyn Fn(&str) -> ArgParseResult<T>;

pub struct Opt<'a> {
	names: &'a [&'a str],
	descr: &'a str,
	invoker: &'a dyn Fn() -> ArgParseResult<()>,
	invoked: bool,
}

pub struct ValOpt<'a, T: Sized> {
	names: &'a [&'a str],
	descr: &'a str,
	invoked: bool,
	parser: OptParser<'a, T>,
	value: Option<Box<T>>,
}

pub trait OptTrait {
	fn get_names(&self) -> &[&str];
	fn get_descr(&self) -> &str;
	fn invoke(&mut self) -> ArgParseResult<()>;
	fn parse(&mut self, arg: &str) -> Option<ArgParseResult<()>>;
	fn need_argument(&self) -> bool;
	fn type_name(&self) -> Option<&str>;
	fn reset(&mut self);
}

impl<'life> Opt<'life> {
	pub fn new<'a>(names: &'a [&'a str], descr: &'a str) -> Opt<'a> {
		let res = Opt {
			names,
			descr,
			invoker: &|| ParseOk(()),
			invoked: false
		};
		res
	}
	pub fn need<T>(self, parser: OptParser<'life, T>, default: Option<Box<T>>) -> ValOpt<'life, T> {
		ValOpt {
			parser,
			value: default,
			names: self.names,
			descr: self.descr,
			invoked: self.invoked
		}
	}

	pub fn is_invoked(&self) -> bool{
		self.invoked
	}

	pub fn set_invoker(&mut self, invok: &'life dyn Fn() -> ArgParseResult<()>){
		self.invoker = invok;
	}
}

impl<T: Clone + FromStr> ValOpt<'_, T> {
	pub fn new<'a>(names: &'a[&'a str], descr: &'a str) -> ValOpt<'a, T> {
		ValOpt {
			names,
			descr,
			parser: &crate::parser::any,
			invoked: false,
			value: None
		}
	}
}

impl<'life, T: Clone> ValOpt<'life, T> {
	pub fn need(mut self, parser: OptParser<'life, T>, default: Option<Box<T>>) -> Self {
		self.parser = parser;
		self.value = default;
		self
	}

	pub fn is_invoked(&self) -> bool {
		self.invoked
	}
	pub fn take(&mut self) -> Option<T> {
		let val = self.value.take();
		return match val {
			None => None,
			Some(val) => Some(*val)
		}
	}
}

impl<'life, T: Clone> ValOpt<'life, T> {
	pub fn get(&self) -> Option<T> {
		let val = self.value.as_ref(); 
		return match val {
			None => None,
			Some(val) => Some(val.as_ref().clone())
		}
	}
}

impl<'life> OptTrait for Opt<'life> {
	fn reset(&mut self) {
		self.invoked = false;
	}
	fn get_names(&self) -> &[&str] {
		&self.names
	}
	fn get_descr(&self) -> &str {
		&self.descr
	}
	fn invoke(&mut self) -> ArgParseResult<()> {
		let res = (self.invoker)();
		self.invoked = true;
		return res;
	}
	fn parse(&mut self, _: &str) -> Option<ArgParseResult<()>> {
		None
	}
	fn need_argument(&self) -> bool {
		false
	}
	fn type_name(&self) -> Option<&str> {
		None
	}
}

impl<'life, T: Clone> OptTrait for ValOpt<'_, T> {
	fn reset(&mut self) {
		self.invoked = false;
		self.value = None;
	}
	fn get_names(&self) -> &[&str] {
		&self.names
	}
	fn get_descr(&self) -> &str {
		&self.descr
	}
	fn invoke(&mut self) -> ArgParseResult<()> {
		self.invoked = true;
		return ParseOk(());
	}
	fn parse(&mut self, arg: &str) -> Option<ArgParseResult<()>> {
		Some (
			match (self.parser)(arg) {
				ParseOk(v) => {self.value = Some(Box::new(v)); ParseOk(())},
				DispHelp => DispHelp,
				TypeError(s) => TypeError(s)
			}
		)
	}
	fn need_argument(&self) -> bool {
		true
	}
	fn type_name(&self) -> Option<&str> {
		Some(type_name::<T>())
	}
}
