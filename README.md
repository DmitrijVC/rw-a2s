# rw-a2s debug
A2S lib for Rust-Wipe.com <br>
Debug branch is for testing purposes only!

## Issues
- Regions different than ALL, are not working
- Port should be u16 not u32
- ~~Info and MasterServers should be moved to client~~ *(fixed in v0.2.0-debug)*

## Example of v0.2.0-debug
Code:
```rust
use rw_a2s::net::server::{Server, Info};
use rw_a2s::net::client::{Client, MasterServers};
use rw_a2s::net::client::filters::{Filter, FilterCode, Regions};
use rw_a2s::types::Bool;
use rw_a2s::errors::ServerError;

fn get_server_info(ip: String, port: u32) -> Result<Info, ServerError> {
    let server = Server::new(ip, port)?;
    server.get_info()
}

fn main() {
    let mut client = Client::new().unwrap();
    client.connect_to_master(MasterServers::Source.get_host()).unwrap();

    let mut filters = Filter::new(None);
    filters.add_unchecked(FilterCode::AppId, &252490_u32);
    filters.add_unchecked(FilterCode::Secure, &Bool::TRUE);
    filters.add_unchecked(FilterCode::Full, &Bool::TRUE);

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
-> [46.29.21.219:28030] esssa.maxcraft.pl
-> [83.23.110.80:28015] Vanilliowa Wies - map 3500
-> [77.79.52.70:28015] [EU] Rustiak | Small Map | 2x Resources |
-> [85.232.145.99:28015] Build server
-> [51.77.61.229:28752] Polski Rust dla giga koxow
-> [51.83.143.167:28245] WePlayRust
-> [51.77.57.19:28015] [RU] Facepunch 4
-> [51.38.148.45:25579] <OFFLINE>
-> [51.38.147.22:28015] [EU/PL] Zamotani PVE |07.01|Raid|4500|Decay 30%|Vanilla|
-> [51.38.134.175:28015] [EU-EAST] Szwajcaria Podlasia
-> [54.38.49.12:28015] [PL] Serwer na czilku :)
-> [137.74.4.84:28015] SideGaming
-> [54.36.175.96:25567] Shockbyte Rust Server
-> [51.38.148.45:25575] <OFFLINE>
-> [51.77.53.113:25566] WIZLA'S WASTELAND NEW IP 51.89.180.186:28015
-> [54.38.195.130:25598] <OFFLINE>
-> [51.38.148.38:25582] <OFFLINE>
-> [213.32.122.242:28759] Twoj serwer @LiveServer.pl
-> [188.165.22.92:28752] Vexus serwer @LiveServer.pl
-> [51.75.52.211:25582] <OFFLINE>
-> [51.83.247.169:28025] <OFFLINE>
-> [137.74.32.104:2042] RustyOmega 2X Everything | Multiple Plugins | New Server | Come
-> [54.37.129.48:28015] [EU] CoolRust MAIN 2x | Fullwiped: 12.01 16:00 CET
-> [51.77.57.35:28015] <OFFLINE>
-> [54.36.175.67:25576] <OFFLINE>
-> [137.74.4.131:28225] Rice Hills [Modded | x2] | Last Wipe: 7 Jan 2021
...
```
