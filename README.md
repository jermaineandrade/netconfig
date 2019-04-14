# netconfig
Simple command line tool for network interfaces

## Usage

### Help Menu
```
jermaines-mbp:~ jermaine$ ./netconfig -h
Network Interface Command Line Tool 1.0
Jermaine Andrade
Retrieve basic network interface information

USAGE:
    netconfig [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --device <INTERFACE NAME>
jermaines-mbp:~ jermaine$
```

### Device Information
```
jermaines-mbp:~ jermaine$ ./netconfig -d en6
en6
Flags: <up,multicast,running>
Inet [fe80::aede:48ff:fe00:1122]:0
Link ac:de:48:00:11:22

jermaines-mbp:~ jermaine$
```
