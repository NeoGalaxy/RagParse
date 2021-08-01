use std::str::FromStr;
use crate::parser::{ArgType, type_name};

pub type ArgParser<'a,T> = &'a dyn Fn(&str) -> Result<Box<T>, String>;
pub struct Arg<'a, T: ?Sized> {
    name: &'a str,
    descr: &'a str,
    parser: ArgParser<'a, T>,
    required: bool,
    value: Option<Box<T>>
}

pub trait ArgTrait {
    fn get_name(&self) -> &str;
    fn get_descr(&self) -> &str;
    fn parse(&mut self, arg: &str) -> Result<(), String>;
    fn is_required(&self) -> bool;
    fn type_name(&self) -> &str;
}

impl<T: Clone + FromStr + ArgType> Arg<'_, T> {
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

impl<'life, T: Clone> Arg<'life, T> {
    pub fn set_parser(mut self, parser: ArgParser<'life, T>, default: Option<Box<T>>) -> Self {
        self.parser = parser;
        self.value = default;
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

impl<T: Clone + ArgType> ArgTrait for Arg<'_, T> {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_descr(&self) -> &str {
        &self.descr
    }
    fn parse(&mut self, arg: &str) -> Result<(), String> {
        match (self.parser)(arg) {
            Ok(value) => {
                self.value = Some(value);
                return Ok(());
            },
            Err(errmsg) => return Err(errmsg),
        }
    }
    fn is_required(&self) -> bool {
        self.required
    }
    fn type_name(&self) -> &str {
        type_name::<T>()
    }
}

