// Constructed for Rust-Game servers, without the challenge number
// See https://developer.valvesoftware.com/wiki/Server_queries

use crate::net::ToUdpSocket;
use crate::errors::ServerError;
use std::net::SocketAddr;
use std::time::Duration;
use std::cell::Cell;

const PREFIX_INFO_RESPONSE: [u8; 6] = [0xFF, 0xFF, 0xFF, 0xFF, 0x49, 0x11];
const INFO_PACKET: [u8; 25] = [0xFF, 0xFF, 0xFF, 0xFF, 0x54, 0x53, 0x6F, 0x75, 0x72, 0x63, 0x65, 0x20, 0x45, 0x6E, 0x67, 0x69, 0x6E, 0x65, 0x20, 0x51, 0x75, 0x65, 0x72, 0x79, 0x00];


fn read_string(start: usize, data: &Vec<u8>) -> String {
    let d = &data[start..];
    String::from_utf8_lossy(&d[..d.iter().position(|&x| x == 0).unwrap() as usize]).to_string()
}


pub struct Server<T: ToUdpSocket> {
    ip: String,
    port: u16,
    pub socket: T,
} impl<T: ToUdpSocket> Server<T> {

    // ToDo fix checking if the host is offline
    pub fn new(ip: String, port: u16, socket: T) -> Result<Self, ServerError> {
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

    pub fn set_write_timeout(self, dur: Duration) -> Self {
        let _ = self.socket.write_timeout(Some(dur));
        self
    }

    pub fn set_read_timeout(self, dur: Duration) -> Self {
        let _ = self.socket.read_timeout(Some(dur));
        self
    }

    fn connect(&mut self) -> bool {
        return match self.socket.conn(SocketAddr::new(
            self.ip.parse().unwrap(),
            self.port as u16
        ) ) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn get_info(&self) -> Result<Info, ServerError> {
        self.socket.send_packet(&INFO_PACKET).unwrap();

        let mut buf = [0; 4096];
        let response = match self.socket.receive_packet(&mut buf) {
            Ok(result) => result,
            Err(_) => {
                return Err(ServerError::TimedOut);
            },
        };

        // println!("{}", String::from_utf8_lossy(&buf[..response]));
        Info::new_from_raw(buf[..response].to_vec(), self.ip.clone(), self.port as u16)
    }
}

#[derive(Debug)]
pub enum ServerType {
    Dedicated,
    NonDedicated,
    SourceTV,
    Unknown
}

impl From<u8> for ServerType {
    fn from(v: u8) -> Self {
        match v as char {
            'd' => ServerType::Dedicated,
            'l' => ServerType::NonDedicated,
            'p' => ServerType::SourceTV,
            _ => ServerType::Unknown
        }
    }
}

#[derive(Debug)]
pub enum ServerEnvironment {
    Linux,
    Windows,
    Mac,
    Unknown
}

impl From<u8> for ServerEnvironment {
    fn from(v: u8) -> Self {
        match v as char {
            'l' => ServerEnvironment::Linux,
            'w' => ServerEnvironment::Windows,
            'm' | 'o' => ServerEnvironment::Mac,
            _ => ServerEnvironment::Unknown
        }
    }
}

#[derive(Debug)]
pub struct Info {
    pub ip: String,
    pub port: u16,
    pub name: String,
    pub map: String,
    pub folder: String,
    pub game: String,
    pub appid: u16,
    pub players: u8,
    pub max_players: u8,
    pub bots: u8,
    pub server_type: ServerType,
    pub environment: ServerEnvironment,
    pub visibility: bool, // false if private
    pub vac: bool,
    pub version: String,
    pub extra_data: EDF
}

impl Info {
    pub fn new_from_raw(data: Vec<u8>, ip: String, port: u16) -> Result<Self, ServerError>{
        if data.starts_with(&PREFIX_INFO_RESPONSE) {
            let data_pointer = Cell::new(PREFIX_INFO_RESPONSE.len());

            let read_string = || {
                let d = read_string(data_pointer.get(), &data);
                data_pointer.set(data_pointer.get() + d.len() + 1);
                d
            };

            let read_short = || {
                let d = u16::from_le_bytes([data[data_pointer.get()], data[data_pointer.get() + 1]]);
                data_pointer.set(data_pointer.get() + 2);
                d
            };

            let read_byte = || {
                let d = data[data_pointer.get()];
                data_pointer.set(data_pointer.get() + 1);
                d
            };

            let read_server_type = || {
                let t = ServerType::from(data[data_pointer.get()]);
                data_pointer.set(data_pointer.get() + 1);
                t
            };
            let read_environment = ||{
                let e = ServerEnvironment::from(data[data_pointer.get()]);
                data_pointer.set(data_pointer.get() + 1);
                e
            };

            Ok(Self {
                ip,
                port,
                name: read_string(),
                map: read_string(),
                folder: read_string(),
                game: read_string(),
                appid: read_short(),
                players: read_byte(),
                max_players: read_byte(),
                bots: read_byte(),
                server_type: read_server_type(),
                environment: read_environment(),
                visibility: read_byte() == 0,
                vac: read_byte() != 0,
                version: read_string(),
                extra_data: EDF::from(&data[data_pointer.get()..])
            })
        } else {
            Err(ServerError::InvalidResponsePrefix)
        }
    }
}


#[derive(Debug)]
pub struct EDF {
    pub port: Option<u16>,
    pub steam_id: Option<u64>,
    pub source_tv_port: Option<u16>,
    pub source_tv_name: Option<String>,
    pub tags: Option<String>,
    pub game_id: Option<u64>
}

impl From<&[u8]> for EDF {
    fn from(data: &[u8]) -> Self {
        let flags = data[0];
        
        let mut dp = 1_usize;

        let port = if flags&0x80 != 0 {
            let t = u16::from_le_bytes([data[1], data[2]]);
            dp += 2;
            Some(t)
        } else { None };

        let steam_id = if flags&0x10 != 0 {
            let t = u64::from_le_bytes([data[dp], data[dp+1], data[dp+2], data[dp+3], data[dp+4], data[dp+5], data[dp+6], data[dp+7]]);
            dp += 8;
            Some(t)
        } else { None };

        let (source_tv_port, source_tv_name) = if flags&0x40 != 0 {
            let port = u16::from_le_bytes([data[dp], data[dp+1]]);
            let name = read_string(dp+2, &data.to_vec());
            dp += 3 + name.len();
            (Some(port), Some(name))
        } else { (None, None) };

        let tags = if flags&0x20 != 0 {
            let t = read_string(dp, &data.to_vec());
            dp += t.len() + 1;
            Some(t)
        } else { None };

        
        let game_id = if flags&0x01 != 0 {
            Some(u64::from_le_bytes([data[dp], data[dp+1], data[dp+2], data[dp+3], data[dp+4], data[dp+5], data[dp+6], data[dp+7]]))
        } else { None };

        Self {
            port,
            steam_id,
            source_tv_port,
            source_tv_name,
            tags,
            game_id
        }
    }
}
