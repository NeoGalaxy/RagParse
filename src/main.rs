use argparse::{arg::{Arg, ArgTrait},opt::{Opt, ValOpt, OptTrait}};

fn main() {
    let data: Vec<String> = std::env::args().collect();

    let mut name_arg: Arg<String> = Arg::new("name", "The name of the element");
    let mut age_arg: Arg<i16> = Arg::new("age", "The age of the element");
    
    let mut h_opt: Opt = Opt::new(&["h", "-help"], "Display the help");
    let mut wd_opt: ValOpt<i32> = ValOpt::new(&["wd"], "A random option");
    let helpcl;

    let all_args:&mut [&mut dyn ArgTrait] = &mut [&mut name_arg, &mut age_arg];
    {
        let all_opts:&mut [&mut dyn OptTrait] = &mut [&mut h_opt, &mut wd_opt];
        let ret = argparse::doc::help(all_args, all_opts, &data[0]);
        helpcl = move || {println!("{}", ret); std::process::exit(0)};
        h_opt.set_invoker(&helpcl);
    }
    let all_opts:&mut [&mut dyn OptTrait] = &mut [&mut h_opt, &mut wd_opt];


    argparse::parse(all_args, all_opts, &data);

    println!("{:?}", h_opt.is_invoked());
    println!("{:?}", wd_opt.is_invoked());
}
