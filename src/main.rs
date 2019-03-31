extern crate clap;

use clap::{Arg, App};

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

    let device_name = matches.value_of("device").unwrap();
    println!("Specified device: {}", device_name);
}
