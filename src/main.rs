extern crate clap;
extern crate nix;
extern crate libc;

use std::collections::HashMap;
use clap::{Arg, App};
use nix::ifaddrs::InterfaceAddress;
use nix::net::if_::InterfaceFlags;

fn main() {
    let matches = App::new("Network Interface Command Line Tool")
                            .version("1.0")
                            .author("Jermaine Andrade")
                            .about("Retrieve basic network interface information")
                            .arg(Arg::with_name("device")
                                .short("d")
                                .long("device")
                                .value_name("INTERFACE NAME"))
                            .get_matches();

    //TODO Error handling around argument handling
    let device_name = matches.value_of("device").unwrap();
    //println!("Specified device: {}", device_name);

    let address_iterator = nix::ifaddrs::getifaddrs().unwrap();
    let mut interface_addresses = Vec::new();

    for ifaddr in address_iterator {
        if ifaddr.interface_name == device_name {
            interface_addresses.push(ifaddr);
            //println!("Interface Name: {}, Address {:?}", ifaddr.interface_name, ifaddr);
        }
    }
    compose_interface_output(&interface_addresses[0]);
}

fn compose_interface_output(interface_address: &InterfaceAddress) -> String {
    let flags = parse_interface_flags(&interface_address.flags);
    let flag_output = compose_flag_output(flags);

    println!("Flags: {}", flag_output);
    return String::from(" ");
}

fn parse_interface_flags(flags: &InterfaceFlags) -> HashMap<String, bool> {
    let mut if_flags = HashMap::new();
    if_flags.insert("loopback".to_string(), InterfaceFlags::bits(flags) & libc::IFF_LOOPBACK != 0);
    if_flags.insert("multicast".to_string(), InterfaceFlags::bits(flags) & libc::IFF_MULTICAST != 0);
    if_flags.insert("running".to_string(), InterfaceFlags::bits(flags) & libc::IFF_RUNNING != 0);
    if_flags.insert("up".to_string(), InterfaceFlags::bits(flags) & libc::IFF_UP != 0);

    return if_flags
}

fn compose_flag_output(flags: HashMap<String, bool>) -> String {
    let mut flag_output = String::from("<");
    let mut flags_set_true: Vec<String>  = Vec::new();

    for (flag, has_flag) in flags {
        if has_flag == true {
            //flag_output.push_str(&flag);
            //flag_output.push_str(",")
            flags_set_true.push(flag);
        }
    }
    flag_output.push_str(&flags_set_true.join(","));
    flag_output.push_str(">");
    return flag_output;
}
