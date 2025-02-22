#[cfg(all(feature = "std", feature = "release-build"))]
fn main() {
	substrate_wasm_builder::WasmBuilder::init_with_defaults()
		.enable_metadata_hash("HDX", 12)
		.build();
}

#[cfg(all(feature = "std", not(feature = "release-build")))]
fn main() {
	substrate_wasm_builder::WasmBuilder::build_using_defaults();
}

/// The wasm builder is deactivated when compiling
/// this crate for wasm to speed up the compilation.
#[cfg(not(feature = "std"))]
fn main() {}
