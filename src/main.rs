#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate clap;

mod operations;

fn main() {

    let matches = clap::App::new("tody cli")
		.version("0.1.0")
		.author("Arnold Agus")
		.about("tody cli tool")
	.arg(clap::Arg::with_name("operation")
		.index(1)
		.required(true)
		.possible_values(&["deploy"])
		.help("an operation"))
    .arg(clap::Arg::with_name("targets")
		.index(2)
		.multiple(true)
		.takes_value(true)
		.help("target of previous command"))
	.get_matches();

    match matches.value_of("operation").unwrap() {
		"deploy" => {
			operations::deploy::deploy_all(matches.values_of("targets").unwrap());
		}
		_  => println!("Operation not available.")
	}


}
