// Constructed for Rust-Game servers, without the challenge number
// See https://developer.valvesoftware.com/wiki/Server_queries

pub mod filters;

use crate::errors::ServerError;
use std::time::Duration;
use std::net::{UdpSocket, SocketAddr};

const PREFIX_INFO_RESPONSE: [u8; 6] = [0xFF, 0xFF, 0xFF, 0xFF, 0x49, 0x11];
const INFO_PACKET: [u8; 25] = [0xFF, 0xFF, 0xFF, 0xFF, 0x54, 0x53, 0x6F, 0x75, 0x72, 0x63, 0x65, 0x20, 0x45, 0x6E, 0x67, 0x69, 0x6E, 0x65, 0x20, 0x51, 0x75, 0x65, 0x72, 0x79, 0x00];


pub enum MasterServers {
    Source,
} impl MasterServers {
    pub fn get_host(&self) -> &str {
        match *self {
            MasterServers::Source => "hl2master.steampowered.com:27011",
        }
    }
}

pub struct Server {
    ip: String,
    port: u32,
    socket: UdpSocket,
} impl Server {

    // ToDo fix checking if the host is offline
    pub fn new(ip: String, port: u32) -> Result<Self, ServerError> {
        let socket = match UdpSocket::bind("0.0.0.0:0") {
            Ok(result) => result,
            Err(error) => return Err(ServerError::IoError(error)),
        };

        // ToDo check if Result returned Ok
        let _ = socket.set_write_timeout(Some(
            Duration::from_millis(1000)
        ));

        let _ = socket.set_read_timeout(Some(
            Duration::from_millis(1000)
        ));

        let mut server = Self {
            ip,
            port,
            socket,
        };

        return match server.connect() {
            true => Ok(server),
            false => Err(ServerError::TimedOut)
        }
    }

    fn connect(&mut self) -> bool {
        return match self.socket.connect(SocketAddr::new(
            self.ip.parse().unwrap(),
            self.port as u16
        ) ) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn get_info(&self) -> Result<Info, ServerError> {
        self.socket.send(&INFO_PACKET).unwrap();

        let mut buf = [0; 4096];
        let response = match self.socket.recv(&mut buf) {
            Ok(result) => result,
            Err(_) => {
                return Err(ServerError::TimedOut);
            },
        };

        // println!("{}", String::from_utf8_lossy(&buf[..response]));
        Info::new_from_raw(Data {raw: &buf[..response]}, self.ip.clone(), self.port as u16)
    }
}

pub struct Data<'a> {
    raw: &'a [u8],
}

#[derive(Debug)]
pub struct Info {
    pub ip: String,
    pub port: u16,
    pub name: String,
    pub map_name: String,
    pub wipe_stamp: String,
    pub fps: String,
    pub players_max: String,
    pub players_now: String,
} impl Info {
    fn read_string(start: usize, data: &Data) -> String {
        let mut string = String::new();
        for b in &data.raw[start..] {
            if b != &0x00 {
                string.push(
                    char::from(*b)
                );
            } else {
                break;
            }
        }

        string
    }

    fn read_string_owned(start: usize, data: Data) -> String {
        let mut string = String::new();
        for b in &data.raw[start..] {
            if b != &0x00 {
                string.push(
                    char::from(*b)
                );
            } else {
                break;
            }
        }

        string
    }

    pub fn new_from_raw(data: Data, ip: String, port: u16) -> Result<Self, ServerError>{
        return if data.raw.starts_with(&PREFIX_INFO_RESPONSE) {
            let name = Self::read_string(PREFIX_INFO_RESPONSE.len(), &data);
            let map_name = Self::read_string(PREFIX_INFO_RESPONSE.len() + name.len() + 1, &data);

            let mut extra_fields_i = 0;
            let start = PREFIX_INFO_RESPONSE.len() + name.len() + 1 + map_name.len() + 1;
            for i in start..data.raw.len() - 1 {
                if &data.raw[i] == &0x3F {
                    if &data.raw[i + 1] == &0x40 && &data.raw[i + 2] == &0x01 {
                        extra_fields_i = i + 3;
                        break;
                    }
                }
            }

            let mut extra_fields: Vec<String> = Vec::new();
            for x in Self::read_string_owned(extra_fields_i, data).split(",") {
                extra_fields.push(x.to_string())
            }

            // ToDo implement players_max and players_now
            let mut wipe_stamp = String::from("NaN");
            let mut fps = String::from("NaN");
            let players_max = String::from("NaN");  // unimplemented
            let players_now = String::from("NaN");  // unimplemented

            for field in extra_fields.iter() {
                if field.starts_with("born") {
                    wipe_stamp = field.replace("born", "");
                } else if field.starts_with("fps_avg") {
                    fps = field.replace("fps_avg", "");
                }
            }

            Ok(Self {
                ip,
                port,
                name,
                map_name,
                wipe_stamp,
                fps,
                players_max,
                players_now,
            })
        } else {
            Err(ServerError::InvalidResponsePrefix)
        }
    }
}
