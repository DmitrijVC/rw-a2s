# rw-a2s debug
A2S lib for Rust-Wipe.com <br>
Debug branch is for testing purposes only!

## Issues
- Regions different from ALL, are not working for some apps

## Example of v0.4.0-debug
Code:
```rust
use rw_a2s::*;


fn get_server_info(ip: String, port: u16) -> Result<Info, ServerError> {
    let server = Server::new(ip, port, None)?;
    server.get_info()
}

fn main() {
    let mut client = Client::new(None).unwrap();
    client.connect_to_master(MasterServers::Source.get_host());

    let filters = Filter::new()
        .add(FilterCode::AppId ( 252490 ))
        .add(FilterCode::Secure ( Bool(false) ))
        .add(FilterCode::Full( Bool(true) ));

    client.get_servers(Regions::All, filters, |ip, port| {

        let info = match get_server_info(ip.clone(), port) {
            Ok(result) => result,
            Err(_) => {
                println!("-> [{}:{}] <OFFLINE>", ip, port);
                return;
            }
        };

        println!("-> [{}:{}] {}", ip, port, info.name);
    });
}
```
Output:
```
[MASTER SERVER] received 1392 bytes
-> [77.46.72.161:28015] ShataN Inside | rust.shatan.pl [PvE]
-> [83.28.241.25:28015] <OFFLINE>
-> [46.29.21.219:28030] esssa.maxcraft.pl
-> [137.74.32.104:2042] RustyOmega 2X Everything | Multiple Plugins | New Server | Come
-> [51.38.148.38:25566] RP Server / No PVP unless it is Canon / Bans for non-rp players
-> [137.74.32.104:131] <OFFLINE>
-> [145.239.24.178:27240] [EU] Pumpkins Eaters
-> [51.83.247.169:28025] <OFFLINE>
-> [54.37.129.48:28017] [EU] Bestrust Solo/Duo/Trio 2x | Fullwiped: 17.01 12:00 CET
-> [145.239.133.140:28015] [RU] Facepunch Hapis
-> [51.38.148.45:25571] <OFFLINE>
-> [213.161.99.140:28015] Rust Server
-> [145.239.134.3:1611] <OFFLINE>
-> [51.77.52.122:25578] <OFFLINE>
-> [51.38.148.38:25573] <OFFLINE>
-> [51.38.148.31:25586] <OFFLINE>
-> [54.38.195.104:1607] <OFFLINE>
-> [54.38.195.104:25585] <OFFLINE>
-> [137.74.4.84:28015] SideGaming
-> [51.77.52.123:25573] <OFFLINE>
-> [51.77.53.114:25585] <OFFLINE>
-> [54.36.175.75:25584] <OFFLINE>
-> [51.83.214.131:28015] ! MultiRust | x2 Vanilla | SOLO/DUO | Full Wipe 16/01 | 2x Vani
-> [145.239.134.3:25593] <OFFLINE>
-> [51.38.147.22:28015] [EU/PL] Zamotani PVE |07.01|Raid|4500|Decay 30%|Vanilla|
-> [51.77.52.123:25592] <OFFLINE>
...
```
