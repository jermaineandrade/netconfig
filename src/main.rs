extern crate clap;
extern crate nix;
extern crate libc;

use std::collections::HashMap;
use clap::{Arg, App};
use nix::ifaddrs::InterfaceAddress;
use nix::net::if_::InterfaceFlags;
use std::net::IpAddr;
use nix::sys::socket::SockAddr;

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

    let address_iterator = nix::ifaddrs::getifaddrs().unwrap();
    let mut if_addresses = Vec::new();

    for ifaddr in address_iterator {
        if ifaddr.interface_name == device_name {
            if_addresses.push(ifaddr);
        }
    }
    compose_interface_output(if_addresses);
}

fn compose_interface_output(interface_addresses: Vec<InterfaceAddress>) -> String {
    let mut interface_outputs: Vec<String> = Vec::new();
    let name_output: String = interface_addresses[0].interface_name.clone();
    interface_outputs.push(name_output);
    //println!("{}", name_output);

    let flags: HashMap<String, bool> = parse_interface_flags(&interface_addresses[0].flags);
    let flag_output: String = compose_flag_output(flags);
    interface_outputs.push(flag_output);
    //println!("Flags: {}", flag_output);

    let addresses = parse_interface_addresses(&interface_addresses);
    //let address_output = compose_address_output(addresses);
    //interface_outputs.push(address_output);
    //return interface_outputs.join("\n");
    //println!("{:?}", interface_outputs.join('\n'));
    return String::from(" ");
}

fn parse_interface_flags(flags: &InterfaceFlags) -> HashMap<String, bool> {
    let mut if_flags = HashMap::new();
    if_flags.insert("loopback".to_string(), InterfaceFlags::bits(flags) & libc::IFF_LOOPBACK != 0);
    if_flags.insert("multicast".to_string(), InterfaceFlags::bits(flags) & libc::IFF_MULTICAST != 0);
    if_flags.insert("running".to_string(), InterfaceFlags::bits(flags) & libc::IFF_RUNNING != 0);
    if_flags.insert("up".to_string(), InterfaceFlags::bits(flags) & libc::IFF_UP != 0);

    if_flags
}


fn parse_interface_addresses(if_addresses: &Vec<InterfaceAddress>) -> HashMap<String, String> {
    let mut type_and_address = HashMap::new();

    for if_address_values in if_addresses.iter() {
        let if_address = if_address_values.address.unwrap();
        let mut address_type = String::new();
        
        match if_address {
            SockAddr::Inet(_) => address_type = "Inet".to_string(),
            SockAddr::Link(_) => address_type = "Link".to_string(),
            SockAddr::Unix(_) => address_type = "Unix".to_string(),
            _ => address_type = "".to_string(),
        }
        type_and_address.insert(address_type, if_address.to_string());
    }

    type_and_address
}

fn compose_flag_output(flags: HashMap<String, bool>) -> String {
    let mut flag_output = String::from("<");
    let mut flags_set_true: Vec<String>  = Vec::new();

    for (flag, has_flag) in flags {
        if has_flag == true {
            flags_set_true.push(flag);
        }
    }

    flag_output.push_str(&flags_set_true.join(","));
    flag_output.push_str(">");

    flag_output
}
