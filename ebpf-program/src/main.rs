#![no_std]
#![no_main]

use aya_ebpf::{
    bindings::xdp_action,
    macros::xdp,
    programs::XdpContext,
};
use aya_log_ebpf::info;

const ETH_P_IP: u16 = 0x0800;
const IPPROTO_TCP: u8 = 6;
const HTTP_PORT: u16 = 80;

#[xdp(name="ebpf_program")]
pub fn ebpf_program(ctx: XdpContext) -> u32 {
    match try_ebpf_program(ctx) {
        Ok(ret) => ret,
        Err(_) => xdp_action::XDP_ABORTED,
    }
}

fn try_ebpf_program(ctx: XdpContext) -> Result<u32, ()> {
    let eth_hdr = ctx.load::<aya_ebpf::cty::c_void>(0).map_err(|_| ())?;
    let eth_type = u16::from_be(unsafe { (*(eth_hdr as *const aya_ebpf::bindings::ethhdr)).h_proto });

    if eth_type != ETH_P_IP {
        return Ok(xdp_action::XDP_PASS);
    }

    let ip_hdr = ctx.load::<aya_ebpf::cty::c_void>(aya_ebpf::cty::sizeof(aya_ebpf::bindings::ethhdr)).map_err(|_| ())?;
    let ip_proto = unsafe { (*(ip_hdr as *const aya_ebpf::bindings::iphdr)).protocol };

    if ip_proto != IPPROTO_TCP {
        return Ok(xdp_action::XDP_PASS);
    }

    let tcp_hdr = ctx.load::<aya_ebpf::cty::c_void>(aya_ebpf::cty::sizeof(aya_ebpf::bindings::ethhdr) + (unsafe { (*(ip_hdr as *const aya_ebpf::bindings::iphdr)).ihl } * 4) as usize).map_err(|_| ())?;
    let dest_port = u16::from_be(unsafe { (*(tcp_hdr as *const aya_ebpf::bindings::tcphdr)).dest });

    if dest_port != HTTP_PORT {
        return Ok(xdp_action::XDP_PASS);
    }

    // This is a very basic check for "GET" and "drop" header.
    // A real implementation would need to be much more robust.
    let http_payload_offset = aya_ebpf::cty::sizeof(aya_ebpf::bindings::ethhdr) + (unsafe { (*(ip_hdr as *const aya_ebpf::bindings::iphdr)).ihl } * 4) as usize + (unsafe { (*(tcp_hdr as *const aya_ebpf::bindings::tcphdr)).doff } * 4) as usize;

    let get = "GET";
    let drop_header = "drop:";

    for i in 0..get.len() {
        let byte = ctx.load::<u8>(http_payload_offset + i).map_err(|_| ())?;
        if byte != get.as_bytes()[i] {
            return Ok(xdp_action::XDP_PASS);
        }
    }

    // A very naive search for the drop header.
    // This is not robust and is just for demonstration purposes.
    for i in 0..100 {
        let mut found = true;
        for j in 0..drop_header.len() {
            let byte = ctx.load::<u8>(http_payload_offset + i + j).map_err(|_| ())?;
            if byte != drop_header.as_bytes()[j] {
                found = false;
                break;
            }
        }
        if found {
            info!(&ctx, "dropping packet with drop header");
            return Ok(xdp_action::XDP_DROP);
        }
    }

    Ok(xdp_action::XDP_PASS)
}
