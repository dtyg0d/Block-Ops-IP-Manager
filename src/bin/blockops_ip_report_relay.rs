use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    env,
    hash::{Hash, Hasher},
    net::{IpAddr, SocketAddr, UdpSocket},
    process,
    time::{Duration, Instant},
};

const DEFAULT_IP_REPORT_PORT: u16 = 14235;
const RELAY_PACKET_PREFIX: &[u8] = b"BLOCKOPS_RELAY_V1\n";
const DUPLICATE_WINDOW: Duration = Duration::from_millis(750);

fn parse_port(value: Option<&String>, name: &str, default: u16) -> u16 {
    match value {
        Some(value) => value.parse::<u16>().unwrap_or_else(|_| {
            eprintln!("Invalid {name}: {value}");
            process::exit(2);
        }),
        None => default,
    }
}

fn packet_fingerprint(sender: IpAddr, packet: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    sender.hash(&mut hasher);
    packet.hash(&mut hasher);
    hasher.finish()
}

fn build_relay_packet(sender: IpAddr, payload: &[u8]) -> Vec<u8> {
    let sender_ip = sender.to_string();
    let mut relayed =
        Vec::with_capacity(RELAY_PACKET_PREFIX.len() + sender_ip.len() + payload.len() + 1);
    relayed.extend_from_slice(RELAY_PACKET_PREFIX);
    relayed.extend_from_slice(sender_ip.as_bytes());
    relayed.push(b'\n');
    relayed.extend_from_slice(payload);
    relayed
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let manager_ip = args.get(1).and_then(|value| value.parse::<IpAddr>().ok());

    let Some(manager_ip) = manager_ip else {
        let program = args
            .first()
            .map(String::as_str)
            .unwrap_or("BlockOps_IP_Report_Relay.exe");
        eprintln!(
            "Usage: {program} <manager-vpn-ip> [rack-listen-port] [manager-port]\n\
             Example: {program} 100.64.0.25"
        );
        process::exit(2);
    };

    let listen_port = parse_port(args.get(2), "rack listen port", DEFAULT_IP_REPORT_PORT);
    let manager_port = parse_port(args.get(3), "manager port", listen_port);
    let manager = SocketAddr::new(manager_ip, manager_port);

    let listener = UdpSocket::bind(("0.0.0.0", listen_port)).unwrap_or_else(|error| {
        eprintln!("Could not listen on UDP {listen_port}: {error}");
        process::exit(1);
    });
    let forwarder = UdpSocket::bind(("0.0.0.0", 0)).unwrap_or_else(|error| {
        eprintln!("Could not open the VPN forwarding socket: {error}");
        process::exit(1);
    });

    println!("BlockOps IP Report Relay 3.0.0");
    println!("Rack listener : UDP 0.0.0.0:{listen_port}");
    println!("VPN manager   : UDP {manager}");
    println!("Keep this window open. Press Ctrl+C to stop.\n");

    let mut packet = [0u8; 4096];
    let mut recently_forwarded: HashMap<u64, Instant> = HashMap::new();

    loop {
        let (length, sender) = match listener.recv_from(&mut packet) {
            Ok(received) => received,
            Err(error) => {
                eprintln!("Rack receive error: {error}");
                continue;
            }
        };

        let payload = &packet[..length];
        if payload.starts_with(RELAY_PACKET_PREFIX) {
            continue;
        }

        let now = Instant::now();
        recently_forwarded.retain(|_, seen| now.duration_since(*seen) < DUPLICATE_WINDOW);
        let fingerprint = packet_fingerprint(sender.ip(), payload);
        if recently_forwarded.contains_key(&fingerprint) {
            continue;
        }
        recently_forwarded.insert(fingerprint, now);

        let relayed = build_relay_packet(sender.ip(), payload);

        match forwarder.send_to(&relayed, manager) {
            Ok(_) => println!("Forwarded IP Report from {sender} to {manager}"),
            Err(error) => eprintln!("Could not forward report from {sender}: {error}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::build_relay_packet;
    use std::net::{IpAddr, Ipv4Addr};

    #[test]
    fn envelope_preserves_sender_and_payload() {
        let packet = build_relay_packet(
            IpAddr::V4(Ipv4Addr::new(10, 4, 7, 42)),
            b"mac=AA-BB-CC-DD-EE-FF",
        );
        assert_eq!(
            packet,
            b"BLOCKOPS_RELAY_V1\n10.4.7.42\nmac=AA-BB-CC-DD-EE-FF"
        );
    }
}
