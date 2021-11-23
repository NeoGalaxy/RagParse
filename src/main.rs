use argparse::{arg, opt, parse_cmd};

fn main() {
    let mut name_arg   = arg!(name? (String): "The name of the subject");
    let mut age_arg    = arg!(age (i16): "The age of the subject");
    let mut weight_arg = arg!(weight? (f64): "The weight of the subject");
    
    let mut h_opt      = opt!([h, "-help"]: "Display the help", invoke(help));
    let mut ro_opt     = opt!([ro, RO, "-random-option"] i32: "A random option");

    parse_cmd!([name_arg, age_arg, weight_arg] <h_opt, ro_opt>);

    match name_arg.get_opt() {
        Some(val) => println!("We are taking about the cat named {}.", val),
        None => println!("I don't know her name, but..."),
    }
    if age_arg.get() < 11 /* the age is already parsed and cast-ed */ {
        print!("She is not a senior yet");
    } else {
        print!("She is a senior");
    }
    match weight_arg.get_opt() {
        Some(val) => println!(" and her weight is about {}kg.", val),
        None => println!("."),
    }
    match ro_opt.get() {
        Some(val) => println!("{}. Wow that was random.", val * 3 - 1),
        None => (),
    }
}
