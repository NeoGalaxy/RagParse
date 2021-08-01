use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap,
};
pub mod parser;
pub mod arg;
pub mod opt;
pub mod doc;

pub enum ParseResult {
    ParseOk,
    OptError,
    TooManyArgs,
    NotEnoughArgs,
    TypeError(String),
}
use ParseResult::*;

pub fn parse_res<'a>(args: &mut [&mut dyn arg::ArgTrait], opts: &mut [&mut dyn opt::OptTrait], data: &[String]) -> ParseResult
{
    /////////   Parsing Options   /////////
    // Registering
    let mut opt_map = HashMap::new();
    for opt in opts {
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
        opt.invoke();
        if opt.need_argument() {
            let second_arg = &data[data_i];
            data_i += 1;
            match opt.parse(second_arg) {
                Ok(()) => (),
                Err(err) => match err {
                    Some(s) => return TypeError(s),
                    None => panic!("{:?}", 1),
                }
            };
        }
    }

    /////////  Parsing Arguments  /////////
    // Check number of arguments
    let mut min_lenght = 0;
    for arg in args.iter() {
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
        if args_i >= args.len() {
            break {
                if data_i < remaining_data.len() {TooManyArgs}
                else {ParseOk}
            };
        }
        let arg = &mut *args[args_i];
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
            Ok(()) => (),
            Err(s) => return TypeError(s),
        }
        data_i += 1;
    }
}

pub fn parse<'a>(args: &mut [&mut dyn arg::ArgTrait], opts: &mut [&mut dyn opt::OptTrait], data: &[String])
{
    match parse_res(args, opts, data) {
        ParseOk => (),
        TooManyArgs => panic!("Too many args"),
        TypeError(s) => panic!("TypeError: {}", s),
        NotEnoughArgs => panic!("NotEnoughArgs"),
        OptError => panic!("Unknown option"),
    }
}

/*#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}*/
