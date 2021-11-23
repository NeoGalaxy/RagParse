use crate::{arg::ArgTrait, opt::OptTrait};

pub fn help<'a>(args: &[&mut dyn ArgTrait], opts: &[&mut dyn OptTrait], cmd: &str) -> String{
	let mut help_string = String::new();

	/************************   Print Usage   ************************/

	help_string.push_str(&format!("Usage:{}", cmd));
	if opts.len() > 0 {
		help_string.push_str(" [OPTIONS]");
	}
	for arg in args.iter() {
		if arg.is_required() {
			help_string.push_str(&format!(" <{}>", arg.get_name()));
		} else {
			help_string.push_str(&format!(" [{}]", arg.get_name()));
		}
	}
	help_string.push_str("\n\n");

	/************************   Print Args   ************************/
	
	if args.len() > 0 {
		help_string.push_str("Arguments:\n");
		for arg in args.iter() {
			help_string.push_str(
				&format!("  {} <{}>\t{}\n", arg.get_name(), arg.type_name(), arg.get_descr())
			)
		}
	}
	if opts.len() > 0 {

		help_string.push_str("Options:\n");
		for opt in opts.iter() {
			let type_name = opt.type_name();
			help_string.push_str("  ");
			let mut first = true;
			for name in opt.get_names() {
				if !first {
					help_string.push_str(", ");
				} else {
					first = false;
				}
				help_string.push_str(&format!("-{}", name));
				if let Some(t_name) = type_name {
					help_string.push_str(&format!(" <{}>", t_name));
				}
			}
			help_string.push_str(&format!("\t{}\n", opt.get_descr()));
		}
	}
	help_string
}
