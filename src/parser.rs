pub mod doc;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::{arg, opt};

#[derive(Debug)]
pub enum ArgParseResult<T> {
	ParseOk(T),
	DispHelp,
	TypeError(String),
}

pub fn any<T>(arg: &str) -> ArgParseResult<T>
	where T:std::str::FromStr
{
	match arg.parse() {
		Ok(value) => ArgParseResult::ParseOk(value),
		Err(_) => ArgParseResult::TypeError(format!("Can't parse argument into {}.", std::any::type_name::<T>())),
	}
}

pub enum ParseResult {
	ParseOk,
	DispHelp,
	OptError,
	TooManyArgs,
	NotEnoughArgs,
	TypeError(String),
}
use ParseResult::*;

pub struct Parser<'a> {
	args: Vec<&'a mut (dyn arg::ArgTrait + 'a)>,
	opts: Vec<&'a mut (dyn opt::OptTrait + 'a)>,
	cmd: Option<String>,
	//error_help: bool
}

impl ParseResult {
	pub fn disp_result(self, parser: Parser) -> bool {
		match self {
			ParseOk => return true,
			DispHelp => (),
			TooManyArgs => println!("Error: Too many args given."),
			TypeError(s) => panic!("TypeError: {}", s),
			NotEnoughArgs => println!("Error: Not enough args given."),
			OptError => println!("Error: Unknown option"),
		};
		println!("{}", doc::help(&parser.args[..], &parser.opts[..], &parser.cmd.unwrap()));
		return false;
	}
}

impl<'life> Parser<'life> {
	pub fn new() -> Parser<'life> {
		Parser {
			cmd: None,
			args: Vec::new(),
			opts: Vec::new(),
			//error_help: true
		}
	}
	pub fn arg<'a>(mut self, arg: &'life mut (dyn arg::ArgTrait + 'a)) -> Self {
		self.args.push(arg);
		self
	}

	pub fn opt<'a>(mut self, opt: &'life mut (dyn opt::OptTrait + 'a)) -> Self {
		self.opts.push(opt);
		self
	}

	pub fn parse(&mut self, data: &'life [std::string::String]) -> ParseResult
	{
		self.cmd = Some(data[0].to_string());
		/////////   Parsing Options   /////////
		// Registering
		let mut opt_map = HashMap::new();
		for opt in &mut self.opts {
			opt.reset();
			let mut names: Vec<String> = Vec::new();
			for name in opt.get_names() {
				names.push(name.to_string());
			}
			let opt_ref = Rc::new(RefCell::new(&mut *opt));
			for name in names {
				opt_map.insert(name.to_string(), opt_ref.clone());
			}
		}
		// Reading
		let mut remaining_data: Vec<&str> = Vec::new();
		let mut data_i = 1;
		while data_i < data.len() {
			let arg = &data[data_i];
			data_i += 1;
			if arg.chars().next().unwrap() != '-' {
				remaining_data.push(&*arg);
				continue;
			}
			let mut opt = match opt_map.get_mut(&arg[1..]) {
				Some(opt) => opt.borrow_mut(),
				None => return OptError,
			};
			match opt.invoke() {
				ArgParseResult::ParseOk(()) => (),
				ArgParseResult::DispHelp => return DispHelp, // TODOOOOOOOO!!!!!!!!!!!!!!
				ArgParseResult::TypeError(s) => return TypeError(s),
			}
			if opt.need_argument() {
				let second_arg = &data[data_i];
				data_i += 1;
				match opt.parse(second_arg).unwrap() {
					ArgParseResult::ParseOk(()) => (),
					ArgParseResult::DispHelp => return DispHelp,
					ArgParseResult::TypeError(err) => return TypeError(err)
				};
			}
		}

		/////////  Parsing Arguments  /////////
		// Check number of arguments
		let mut min_lenght = 0;
		for arg in &mut self.args {
			arg.reset();
			if arg.is_required() {
				min_lenght += 1;
			}
		}
		let mut remaining: i32 = (remaining_data.len() as i32) - min_lenght; // safety net
		if remaining < 0 {
			return NotEnoughArgs;
		}
		let mut data_i = 0;
		let mut args_i = 0;
		// reading
		loop {
			/*** setting up the arg value ***/
			if args_i >= self.args.len() {
				break {
					if data_i < remaining_data.len() {TooManyArgs}
					else {ParseOk}
				};
			}
			let arg = &mut *self.args[args_i];
			args_i += 1;
			if !arg.is_required() {
				if remaining == 0 {
					continue;
				}
				remaining -= 1
			}
			if data_i >= remaining_data.len() {
				panic!("Internal error. Remaining was {}", remaining);
			}
			match arg.parse(&remaining_data[data_i]) {
				ArgParseResult::ParseOk(()) => (),
				ArgParseResult::DispHelp => return DispHelp,
				ArgParseResult::TypeError(s) => return TypeError(s),
			}
			data_i += 1;
		}
	}

	pub fn full_parse<'a>(mut self, data: &'life [std::string::String])
	{
		let ret = self.parse(data).disp_result(self);
		if !ret {
			std::process::exit(0);
		}
	}

}

