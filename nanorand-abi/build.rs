extern crate cbindgen;

use std::path::PathBuf;

fn main() {
	let crate_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

	cbindgen::Builder::new()
		.with_config(
			cbindgen::Config::from_file(crate_dir.join("cbindgen.toml"))
				.expect("Unable to read cbindgen config"),
		)
		.with_crate(crate_dir)
		.generate()
		.expect("Unable to generate bindings")
		.write_to_file("nanorand.h");
}
