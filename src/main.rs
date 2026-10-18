//! Find Jellyfin servers on the local network.
//!
//! Jellyfin listens on UDP port 7359 for the message "who is JellyfinServer?"
//! and replies with a JSON blob containing its name, ID, and URL.

use std::collections::HashMap;
use std::net::UdpSocket;
use std::process::exit;
use std::time::Duration;

use serde_json::Value;

const DISCOVERY_PORT: u16 = 7359;
const MESSAGE: &[u8] = b"who is JellyfinServer?";

fn discover(timeout: Duration) -> std::io::Result<Vec<Value>> {
    let sock = UdpSocket::bind("0.0.0.0:0")?;
    sock.set_broadcast(true)?;
    sock.set_read_timeout(Some(timeout))?;
    sock.send_to(MESSAGE, ("255.255.255.255", DISCOVERY_PORT))?;

    let mut servers: HashMap<String, Value> = HashMap::new();
    let mut buf = [0u8; 4096];
    // Keep reading until the timeout fires.
    while let Ok((n, addr)) = sock.recv_from(&mut buf) {
        let Ok(info) = serde_json::from_slice::<Value>(&buf[..n]) else {
            continue;
        };
        let key = info["Id"]
            .as_str()
            .map(String::from)
            .unwrap_or_else(|| addr.ip().to_string());
        servers.insert(key, info);
    }
    Ok(servers.into_values().collect())
}

fn public_info(address: &str) -> Option<Value> {
    ureq::get(&format!("{address}/System/Info/Public"))
        .timeout(Duration::from_secs(5))
        .call()
        .ok()?
        .into_json()
        .ok()
}

fn main() {
    let mut servers = discover(Duration::from_secs(3)).unwrap_or_else(|e| {
        eprintln!("Discovery failed: {e}");
        exit(1);
    });
    if servers.is_empty() {
        println!("No Jellyfin servers found.");
        exit(1);
    }

    for s in &mut servers {
        let address = s["Address"].as_str().unwrap_or("").to_string();
        if let Some(pi) = public_info(&address) {
            s["version"] = pi.get("Version").cloned().unwrap_or(Value::from(""));
            s["productName"] = pi["ProductName"].clone();
            s["os"] = pi["OperatingSystem"].clone();
            s["setupComplete"] = pi["StartupWizardCompleted"].clone();
        }
    }

    println!("{}", serde_json::to_string_pretty(&servers).unwrap());
}
