#[macro_export]
macro_rules! opt {
	(@$name:literal) => {
		$name
	};
	(@$name:expr) => {
		stringify!($name)
	};
	([$($names:expr),+]: $text:literal) => {
		$crate::opt::Opt::new(&[$(opt!(@$names)),+], $text)
	};
	($name:ident: $text:literal) => {
		opt!([$name]: $text)
	};
	([$($names:expr),+]: $text:literal, invoke(help)) => {
		{
			let mut tmp = opt!([$($names),+]: $text);
			tmp.set_invoker(&move || {$crate::parser::ArgParseResult::DispHelp});
			tmp
		}
	};
	($name:ident: $text:literal, invoke(help)) => {
		{
			let mut tmp = opt!($name: $text);
			tmp.set_invoker(&move || {$crate::parser::ArgParseResult::DispHelp});
			tmp
		}
	};

	([$($names:expr),+] $t:ty: $text:literal) => {
		$crate::opt::ValOpt::<$t>::new(&[$(opt!(@$names)),+], $text)
	};
	($name:ident ($t:ty): $text:literal) => {
		opt!([$name] $t: $text)
	};
}

#[macro_export]
macro_rules! arg {
	($name:ident: $text:literal) => {
		$crate::arg::Arg::new(stringify!($name), $text)
	};
	($name:ident ($t:ty): $text:literal) => {
		$crate::arg::Arg::<$t>::new(stringify!($name), $text)
	};
	($name:ident? ($($t:ty)?): $text:literal) => {
		arg!($name ($($t)?): $text).optionnal()
	};
}


#[macro_export]
macro_rules! parse_cmd {
	([$($arg:tt),*] <$($opt:tt),*>) => {
		let data: Vec<String> = std::env::args().collect();
		let parser = $crate::parser::Parser::new()$(.arg(&mut $arg))*$(.opt(&mut $opt))*;
		parser.full_parse(&data);
	};
}
