use pnet_datalink::{interfaces, NetworkInterface};
use std::thread::sleep;
use std::time::Duration;
use std::net::{IpAddr, Ipv4Addr};
use libarp::client::ArpClient;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    print("(⇀‿‿↼)");
    loop {
        let ips = get_default_interface().unwrap().ips;
        for i in &ips {
            if i.is_ipv4() {
                let mask = ip_to_u32(i.mask());
                if mask.is_some() {
                    let mask = mask.unwrap();
                    let mut low_ip = ip_to_u32(i.ip()).unwrap();
                    low_ip &= mask;
                    let mut client = ArpClient::new().unwrap();
                    let router_mac = match client.ip_to_mac(u32_to_ipaddr(low_ip), Some(Duration::from_millis(100))) {
                        Ok(x) => Some(x),
                        Err(_) => {
                            match client.ip_to_mac(u32_to_ipaddr(low_ip+1), Some(Duration::from_millis(100))) {
                                Ok(x) => Some(x),
                                Err(_) => {
                                    match client.ip_to_mac(u32_to_ipaddr(low_ip+(u32::MAX-mask)), Some(Duration::from_millis(100))) {
                                        Ok(x) => Some(x),
                                        Err(_) => {
                                            match client.ip_to_mac(u32_to_ipaddr(low_ip+(u32::MAX-mask+1)), Some(Duration::from_millis(100))) {
                                                Ok(x) => Some(x),
                                                Err(_) => None
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    };
                    if router_mac.is_some() {
                        let router_mac = router_mac.unwrap();
                        let f = OpenOptions::new()
                            .create(true)
                            .write(true)
                            .append(true)
                            .open("data.txt");
                        if f.is_ok() {
                            let mut f = f.unwrap();
                            print("(◕‿‿◕)");
                            for i in 0..(u32::MAX-mask+1) {
                                match client.ip_to_mac(u32_to_ipaddr(low_ip+i), Some(Duration::from_millis(100))) {
                                    Ok(x) => {
                                        let _ = writeln!(f, "{:?} {} {}", chrono::offset::Local::now(), router_mac, x);
                                    },
                                    Err(_) => {}
                                }
                            }
                            print("(⇀‿‿↼)");
                        }
                        else {
                            print(&format!("(╥☁╥ ) err: {:?}", f));
                        }
                    }
                    else {
                        print("(╥☁╥ )");
                    }
                }
            }
        }
        sleep(Duration::from_secs(600));
    }
}

fn get_default_interface() -> Option<NetworkInterface> {
    // Get a vector with all network interfaces found
    let all_interfaces = interfaces();

    // Search for the default interface - the one that is
    // up, not loopback and has an IP.
    let default_interface = all_interfaces
        .iter()
        .find(|e| e.is_up() && !e.is_loopback() && !e.ips.is_empty());

    Some(default_interface?.clone())
}

fn ip_to_u32(ip: IpAddr) -> Option<u32> {
    let ip_string = ip.to_string();
    let numbers = ip_string.split(".").collect::<Vec<&str>>();
    let mut number:u32 = 0;
    for i in 0..3 {
        number += numbers.get(i)?.parse::<u32>().ok()? as u32*2_u32.pow(24-(i as u32*8));
    }
    Some(number)
} 

fn u32_to_ipaddr(ip: u32) -> Ipv4Addr {
    Ipv4Addr::from([
        (ip/2_u32.pow(24)%256) as u8,
        (ip/2_u32.pow(16)%256) as u8,
        (ip/2_u32.pow(8)%256) as u8,
        (ip%256) as u8
    ])
}

fn print(string: &str) {
    print!("\r{}", string);
    let _ = std::io::stdout().flush();
}
