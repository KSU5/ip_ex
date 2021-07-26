extern crate pnet;

use pnet::datalink;

fn main() {
    /* let mut i = 0;
    for iface in datalink::interfaces() {
        println!("{}", i);
        i += 1;
        println!("{:?}", iface.ips);
    }

    //use pnet_datalink::interfaces;

// Get a vector with all network interfaces found */
let all_interfaces = datalink::interfaces();

//println!("{:?}", all_interfaces[5].ips[1]);
println!("{:?}", all_interfaces[5].ips[1].ip());
}