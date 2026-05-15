use std::process::Command;

fn main() {
	let result = Command::new("bun")
		.arg("run")
		.arg("build")
		.status()
		.expect("Failed to run build script");

	if !result.success() {
		panic!("Astro build failed with status {result}");
	}
}
