use std::str::FromStr;
use crate::parser::{ArgType, type_name};

pub type OptParser<'a,T> = &'a dyn Fn(&str) -> Result<Box<T>, String>;

pub struct Opt<'a> {
    names: &'a [&'a str],
    descr: &'a str,
    invoker: &'a dyn Fn() -> (),
    invoked: bool,
}

pub struct ValOpt<'a, T: ?Sized> {
    names: &'a [&'a str],
    descr: &'a str,
    invoked: bool,
    parser: OptParser<'a, T>,
    value: Option<Box<T>>,
}

pub trait OptTrait {
    fn get_names(&self) -> &[&str];
    fn get_descr(&self) -> &str;
    fn invoke(&mut self);
    fn parse(&mut self, arg: &str) -> Result<(), Option<String>>;
    fn need_argument(&self) -> bool;
    fn type_name(&self) -> Option<&str>;
}

impl<'life> Opt<'life> {
    pub fn new<'a>(names: &'a [&'a str], descr: &'a str) -> Opt<'a> {
        let res = Opt {
            names,
            descr,
            invoker: &|| (),
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

    pub fn set_invoker(&mut self, invok: &'life dyn Fn() -> ()){
        self.invoker = invok;
    }
}

impl<T: Clone + FromStr + ArgType> ValOpt<'_, T> {
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

    pub fn is_invoked(&self) -> bool{
        self.invoked
    }

    pub fn get(&self) -> T {
        self.value.as_ref().unwrap().as_ref().clone()
    }
}

impl<'life> OptTrait for Opt<'life> {
    fn get_names(&self) -> &[&str] {
        &self.names
    }
    fn get_descr(&self) -> &str {
        &self.descr
    }
    fn invoke(&mut self) {
        (self.invoker)();
        self.invoked = true;
    }
    fn parse(&mut self, _: &str) -> Result<(), Option<String>> {
        Err(None)
    }
    fn need_argument(&self) -> bool {
        false
    }
    fn type_name(&self) -> Option<&str> {
        None
    }
}

impl<'life, T: Clone + ArgType> OptTrait for ValOpt<'_, T> {
    fn get_names(&self) -> &[&str] {
        &self.names
    }
    fn get_descr(&self) -> &str {
        &self.descr
    }
    fn invoke(&mut self) {
        self.invoked = true
    }
    fn parse(&mut self, arg: &str) -> Result<(), Option<String>> {
        match (self.parser)(arg) {
            Ok(v) => {self.value = Some(v); Ok(())},
            Err(s) => Err(Some(s))
        }
    }
    fn need_argument(&self) -> bool {
        true
    }
    fn type_name(&self) -> Option<&str> {
        Some(type_name::<T>())
    }
}
