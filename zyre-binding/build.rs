extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell Cargo to link to the zyre library
    println!("cargo:rustc-link-lib=zyre");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .whitelist_function("zyre_new")
        .whitelist_function("zyre_destroy")
        .whitelist_function("zyre_uuid")
        .whitelist_function("zyre_name")
        .whitelist_function("zyre_set_name")
        .whitelist_function("zyre_set_header")
        .whitelist_function("zyre_set_verbose")
        .whitelist_function("zyre_set_port")
        .whitelist_function("zyre_set_evasive_timeout")
        .whitelist_function("zyre_set_expired_timeout")
        .whitelist_function("zyre_set_interval")
        .whitelist_function("zyre_set_interface")
        .whitelist_function("zyre_set_endpoint")
        .whitelist_function("zyre_gossip_bind")
        .whitelist_function("zyre_gossip_connect")
        .whitelist_function("zyre_start")
        .whitelist_function("zyre_stop")
        .whitelist_function("zyre_join")
        .whitelist_function("zyre_leave")
        .whitelist_function("zyre_recv")
        .whitelist_function("zyre_whisper")
        .whitelist_function("zyre_shout")
        .whitelist_function("zyre_whispers")
        .whitelist_function("zyre_shouts")
        .whitelist_function("zyre_peers")
        .whitelist_function("zyre_peers_by_group")
        .whitelist_function("zyre_own_groups")
        .whitelist_function("zyre_peer_groups")
        .whitelist_function("zyre_peer_address")
        .whitelist_function("zyre_peer_header_value")
        .whitelist_function("zyre_socket")
        .whitelist_function("zyre_print")
        .whitelist_function("zyre_version")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write bindings");
}
