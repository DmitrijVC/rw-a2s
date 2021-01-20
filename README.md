# rw-a2s debug
A2S lib for Rust-Wipe.com <br>
Debug branch is for testing purposes only!

## Issues
- Regions different from ALL, are not working for some apps
- ~~Client::get_servers returns the same servers after 5 packets~~ *(fixed in v0.5.5-debug ?)*


## Example of v0.5.4-debug
Code:
```rust
#[macro_use] extern crate lazy_static;

use rw_a2s::*;
use std::net::UdpSocket;
use std::time::Duration;

lazy_static!{
    static ref SERVERS_SOCKET: UdpSocket = {
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        socket.set_read_timeout(Some(Duration::from_millis(100))).unwrap();
        socket.set_write_timeout(Some(Duration::from_millis(100))).unwrap();
        socket
    };
}


fn get_server_info(ip: String, port: u16) -> Result<Info, ServerError> {
    let game_server = Server::new(ip, port, &*SERVERS_SOCKET)?;
    game_server.get_info()
}

fn main() {
    let mut client = Client::new(UdpSocket::bind("0.0.0.0:0").unwrap())
        .set_write_timeout(Duration::from_millis(1000))
        .set_read_timeout(Duration::from_millis(1000));

    let filters = Filter::new()
        .add(FilterCode::AppId(252490));

    client.connect_to_master(MasterServers::Source.get_host()).unwrap();
    client.get_servers(Regions::All, filters,|ip, port| {
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
-> [83.28.241.25:28015] <OFFLINE>
-> [145.239.134.3:25574] <OFFLINE>
-> [51.83.143.167:28245] WePlayRust
-> [145.239.133.139:28015] [RU] Facepunch 1
-> [54.36.175.76:25566] Rust PvE Only / No Offline Raid / Wipe 08.01
-> [54.38.195.130:25598] <OFFLINE>
-> [145.239.24.178:27230] lol
-> [51.83.230.151:28025] Rustas |2X Vanilla |max 4| skinbox | Wipe
-> [51.77.61.229:28752] Polski Rust dla giga koxow
-> [145.239.133.139:28017] [RU] Facepunch 2
-> [51.77.61.229:28763] [PL/EU] Never Give Up 15.01 23:00
-> [51.83.219.75:28757] [PL] RUST-EKIPA
-> [51.83.182.155:28012] Thormen |2| Vanilla |Solo/Duo/Trio| Fullwiped 19.01 20:00 CET
-> [178.32.149.184:28756] Twoj serwer @LiveServer.pl
-> [51.75.34.88:28015] [EU] PGCGaming.pl | X5 | MAX 6 | Clans | NoBPs | NEXT WIPE 22.0
-> [51.77.52.122:25578] <OFFLINE>
-> [54.38.195.132:27017] Wintersun - Solo/Duo/Trio Vanilla - Wiped: 19/01 - EU
-> [51.77.61.229:28760] [EU] The Walkers|X2|PVP| [ZOMBIES][SKINS][INSTACRAFT]
-> [51.77.61.229:28757] [PL] [FRDS] - Serwer przyjazny - bez poj... adm
-> [178.32.149.184:28754] [ PL ] GPTS.PL | SERWER TEAMSPEAKOWY #1
-> [51.77.57.35:28015] <OFFLINE>
-> [54.38.195.104:1607] <OFFLINE>
-> [145.239.134.3:25595] <OFFLINE>
-> [54.36.175.75:25584] <OFFLINE>
-> [178.32.149.184:28751] roksa.pl
...
```
