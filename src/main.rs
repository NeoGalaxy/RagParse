use argparse::{arg::{Arg, ArgTrait},opt::{Opt, ValOpt, OptTrait}};

fn main() {
    let data: Vec<String> = std::env::args().collect();

    let mut name_arg: Arg<String> = Arg::new("name", "The name of the subject").optionnal();
    let mut age_arg: Arg<i16> = Arg::new("age", "The age of the subject");
    let mut weight_arg: Arg<f64> = Arg::new("weight", "The weight of the subject").optionnal();
    
    let mut h_opt: Opt = Opt::new(&["h", "-help"], "Display the help");
    let mut ro_opt: ValOpt<i32> = ValOpt::new(&["ro", "RO", "-random-option"], "A random option");

    let helpcl; /* It has to outlive h_opt */
    let all_args:&mut [&mut dyn ArgTrait] = &mut [&mut name_arg, &mut age_arg, &mut weight_arg];
    {
        let all_opts:&mut [&mut dyn OptTrait] = &mut [&mut h_opt, &mut ro_opt];
        let ret = argparse::doc::help(all_args, all_opts, &data[0]);
        helpcl = move || {println!("{}", ret); std::process::exit(0)};
        h_opt.set_invoker(&helpcl);
    }
    let all_opts:&mut [&mut dyn OptTrait] = &mut [&mut h_opt, &mut ro_opt];

    argparse::parse(all_args, all_opts, &data);

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
    if ro_opt.is_invoked() {
        println!("{}. Wow that was random.", ro_opt.get() * 3 - 1);
    }
}
