use std::path::PathBuf;

use clap::Parser;
use xshell::{cmd, Shell};

#[derive(Debug, Parser)]
struct Opt {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    Build(Build),
}

#[derive(Debug, Parser)]
struct Build {
    /// Build the eBPF program in release mode.
    #[clap(long)]
    release: bool,
}

fn main() -> Result<(), anyhow::Error> {
    let opt = Opt::parse();

    let sh = Shell::new()?;

    match opt.command {
        Command::Build(build) => build_ebpf(&sh, build)?,
    }

    Ok(())
}

fn build_ebpf(sh: &Shell, build: Build) -> Result<(), anyhow::Error> {
    let mut args = vec!["bpf", "build", "--ebpf-package", "ebpf-program"];
    if build.release {
        args.push("--release");
    }
    let cmd = cmd!(sh, "cargo").args(&args);

    // To get around permission issues, we'll build the eBPF program
    // in a directory that's owned by the user.
    let target_dir = PathBuf::from("target");
    let target_dir_str = target_dir.to_str().unwrap();

    let _env = sh.push_env("CARGO_TARGET_DIR", target_dir_str);
    cmd.run()?;

    println!("eBPF program built successfully");

    Ok(())
}
