use anyhow::Context;
use aya::{
    programs::{Xdp, XdpFlags},
    Bpf,
};
use clap::Parser;
use tokio::signal;

#[derive(Debug, Parser)]
struct Opt {
    #[clap(short, long, default_value = "eth0")]
    iface: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let opt = Opt::parse();

    // This will build the eBPF program and load it into the kernel.
    let mut bpf = Bpf::load_file("../target/bpfel-unknown-none/debug/ebpf-program")
        .context("failed to load bpf file")?;
    let program: &mut Xdp = bpf.program_mut("ebpf_program").unwrap().try_into()?;
    program.load()?;
    program.attach(&opt.iface, XdpFlags::default())
        .context("failed to attach xdp program")?;

    println!("XDP program attached to interface {}", opt.iface);
    println!("Waiting for Ctrl-C...");
    signal::ctrl_c().await?;
    println!("Exiting...");

    Ok(())
}
