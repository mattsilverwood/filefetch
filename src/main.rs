use std::{any::Any, os::unix::fs::FileTypeExt, path::PathBuf};

use clap::Parser;

#[derive(Parser, Debug)]
struct CmdArgs {
    file: PathBuf,
}

fn main() {
    let args = CmdArgs::parse();

    use std::fs;
    let metadata = fs::metadata(&args.file).expect("Could not grab metadata for this file. Sorry!");

    dbg!(metadata.file_type());
}
