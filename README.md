# Naeio's Rust ArGument parser

> This library is at a very early stage of development, and everything is experimental. Any feedback is would be appreciated !

This library is made to help parsing command line arguments in Rust. To do so, you need to define every argument and option individually, and then give them to the `argparse::parse` function. It automatic converts the command line argument into the desired type. 

My goal is to have a library that would be quite easy to use, but also quite powerful at the same time.

Here, we have a small example of an application of usage `command [name] <age> [weight]`, where `name` is a String, `age` is an integer and `weight` is a float, `name` and `weight` being  optional positional arguments. Additionally, we have two options that can be placed anywhere on the command line.

```rust
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
```

Here are the outputs of this:

```sh
$ ./argparse -h
Usage:./argparse [options] [name] <age> [weight]

Arguments:
  name <string> The name of the subject
  age <int>     The age of the subject
  weight <real> The weight of the subject
Options:
  -h, --help    Display the help
  -ro <int>, -RO <int>, --random-option <int>   A random option

$ ./argparse --help
<same usage message>

$ ./argparse TheName 42 6.9                                               
We are taking about the cat named TheName.
She is a senior and her weight is about 6.9kg.

$ ./argparse 42                                                           
I don't know her name, but...
She is a senior.

$ ./argparse 10                                                           
I don't know her name, but...
She is not a senior yet.

$ ./argparse name 42 -ro 10                                                
We are taking about the cat named name.
She is a senior.
29. Wow that was random.

$ ./argparse Sparkle 12 3 -ro 10                                           
We are taking about the cat named Sparkle.
She is a senior and her weight is about 3kg.
29. Wow that was random.
```
