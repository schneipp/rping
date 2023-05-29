use std::{
    net::{IpAddr, Ipv4Addr},
    time::Duration,
};

use clap::Parser;

//add clap
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, value_parser, default_value = "192.168.0.1")]
    startip: Option<String>,
    #[clap(short, long, value_parser, default_value = "192.168.0.254")]
    endip: Option<String>,
}

struct IPV4;

impl IPV4 {
    fn from_string(input: String) -> Result<IpAddr, anyhow::Error> {
        //split the string in parts
        let split = input.split(".");
        let parts: Vec<u8> = split
            .into_iter()
            .map(|itm| {
                let num = itm.parse::<u8>();
                if let Ok(num) = num {
                    num
                } else {
                    0
                }
            })
            .collect::<Vec<u8>>();
        if parts.len() != 4 {
            return Err(anyhow::anyhow!("Invalid IP"));
        }
        let ip = IpAddr::V4(Ipv4Addr::new(parts[0], parts[1], parts[2], parts[3]));
        Ok(ip)
    }
}

fn main() -> Result<(), anyhow::Error> {
    let args = Cli::parse();
    let ip_start_str = args.startip.unwrap().to_string();
    let ip_end_str = args.endip.unwrap().to_string();
    let ip_start = IPV4::from_string(ip_start_str)?;
    let ip_end = IPV4::from_string(ip_end_str)?;
    ping_range(ip_start, ip_end)?;
    Ok(())
}

fn ping_range(start: IpAddr, end: IpAddr) -> Result<(), anyhow::Error> {
    let res = ping::ping(
        start,
        Some(Duration::from_millis(10)),
        Some(20),
        None,
        None,
        None,
    );
    match res {
        Ok(_x) => {
          //  println!("{} is alive", ip);
        }
        Err(err) => {
            println!("{}", err);
        }
    }
    let ip_start = match start {
        IpAddr::V4(ip) => ip,
        IpAddr::V6(_) => return Err(anyhow::anyhow!("Invalid IP")),
    };
    let ip_end = match end {
        IpAddr::V4(ip) => ip,
        IpAddr::V6(_) => return Err(anyhow::anyhow!("Invalid IP")),
    };
    println!("Checking from {} to {}", ip_start, ip_end);
    for a in 0..255 {
        if a < ip_start.octets()[0] || a > ip_end.octets()[0] {
            continue;
        }
        for b in 0..255 {
            if a == ip_end.octets()[0] && b > ip_end.octets()[1] {
                continue;
            }
            if ip_start.octets()[0] == a {
                if b < ip_start.octets()[1] {
                    continue;
                }
            }
            if ip_end.octets()[0] == a {
                if b > ip_end.octets()[1] {
                    continue;
                }
            }
            //wenn vorheriges oktet ist am ende oder am anfang
            for c in 0..255 {
                if b == ip_end.octets()[1] && c > ip_end.octets()[2] {
                    continue;
                }
                //wenn vorheriges oktet ist am ende oder am anfang
                if ip_start.octets()[1] == b {
                    if c < ip_start.octets()[2] {
                        continue;
                    }
                }
                if ip_end.octets()[1] == b {
                    if c > ip_end.octets()[2] {
                        continue;
                    }
                }

                for d in 1..255 {
                    if c == ip_end.octets()[2] && d > ip_end.octets()[3] {
                        continue;
                    }
                    //wenn vorheriges oktet ist am ende oder am anfang
                    if ip_start.octets()[2] == c {
                        if d < ip_start.octets()[3] {
                            continue;
                        }
                    }
                    if ip_end.octets()[2] == c {
                        if d > ip_end.octets()[3] {
                            continue;
                        }
                    }
                    //println!("Checking {}.{}.{}.{}", a, b, c, d);
                    let ip = Ipv4Addr::new(a, b, c, d);
                    //println!("Pinging {}", ip);
                    let current_ip = IpAddr::V4(ip);
                    let res = ping::ping(
                        current_ip,
                        Some(Duration::from_millis(10)),
                        Some(20),
                        None,
                        None,
                        None,
                    );
                    if let Ok(_x) = res {
                        println!("{} is alive", ip);
                    }
                }
            }
        }
    }
    Ok(())
}
