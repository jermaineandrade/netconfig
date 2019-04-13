extern crate clap;
extern crate nix;
extern crate libc;

use std::collections::HashMap;
use clap::{Arg, App, AppSettings};
use nix::ifaddrs::InterfaceAddress;
use nix::net::if_::InterfaceFlags;
use nix::sys::socket::SockAddr;

fn main() {
    let user_arguments = App::new("Network Interface Command Line Tool")
                            .version("1.0")
                            .author("Jermaine Andrade")
                            .about("Retrieve basic network interface information")
                            .arg(Arg::with_name("device")
                                .short("d")
                                .long("device")
                                .value_name("INTERFACE NAME"))
                            .setting(AppSettings::ArgRequiredElseHelp)
                            .get_matches();

    if let Some(device) = user_arguments.value_of("device") {
        output_device_information(device)
    }
}

fn output_device_information(device_name: &str) {
    let interface_info = get_interface_addresses(device_name.to_string());
    let mut interface_output: String;
    if interface_info.len() == 0 {
        interface_output = "Device information not found.".to_string();
    }
    else {
        interface_output = compose_interface_output(interface_info);
    }
    println!("{}", interface_output)
}

fn get_interface_addresses(if_name: String) -> Vec<InterfaceAddress> {
    let address_iterator = nix::ifaddrs::getifaddrs().unwrap();
    let mut if_addresses = Vec::new();

    for ifaddr in address_iterator {
        if ifaddr.interface_name == if_name {
            if_addresses.push(ifaddr);
        }
    }

    if_addresses
}

fn compose_interface_output(interface_addresses: Vec<InterfaceAddress>) -> String {
    let mut interface_outputs: Vec<String> = Vec::new();
    let name_output: String = interface_addresses[0].interface_name.clone();
    interface_outputs.push(name_output);

    let flags: HashMap<String, bool> = parse_interface_flags(&interface_addresses[0].flags);
    let flag_output: String = compose_flag_output(flags);
    interface_outputs.push(flag_output);

    let types_and_addresses: HashMap<String, String> = parse_interface_addresses(&interface_addresses);
    let address_output: String = compose_address_output(types_and_addresses);
    interface_outputs.push(address_output);

    let interface_output: String =  interface_outputs.join("\n");

    interface_output
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
        let if_address;

        match if_address_values.address {
            Some(sock_addr_type) => if_address = sock_addr_type,
            None => continue,
        }

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

fn compose_address_output(types_and_addresses: HashMap<String, String>) -> String {
    let mut address_output = String::new();

    for (addr_type, address) in types_and_addresses {
        address_output += &(addr_type + " " + &address + "\n");
    }
    address_output
}

fn compose_flag_output(flags: HashMap<String, bool>) -> String {
    let mut flag_output = String::from("Flags: <");
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
