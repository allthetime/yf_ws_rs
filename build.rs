use std::env;
use std::io::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    let mut prost_build = prost_build::Config::new();
    prost_build.file_descriptor_set_path(
        PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR environment variable not set"))
            .join("file_descriptor_set.bin"),
    );

    prost_build.compile_protos(&["src/yaticker.proto"], &["src/"])?;
    Ok(())
}
