pub mod filters;

use crate::net::ToUdpSocket;
use crate::net::client::filters::{Regions, Filter};
use crate::errors::A2SClientError;
use std::net::{ToSocketAddrs};
use std::sync::Mutex;
use lazy_static;
use std::time::Duration;

const MESSAGE_TYPE: u8 = 0x31;
const DEFAULT_HOST: [u8; 9] = [0x30, 0x2E, 0x30, 0x2E, 0x30, 0x2E, 0x30, 0x3A, 0x30];
const NULL_BYTE: u8 = 0x00;
// ToDo implement -> const REPLY_PREFIX: [u8; 6] = [0xFF, 0xFF, 0xFF, 0xFF, 0x66, 0x0A];

lazy_static! {
    static ref HOST: Mutex<Vec<u8>> = {
        Mutex::new(Vec::new())
    };
}


pub enum MasterServers {
    Source,
} impl MasterServers {
    pub fn get_host(&self) -> &str {
        match *self {
            MasterServers::Source => "hl2master.steampowered.com:27011",
        }
    }
}

pub struct Client<S: ToUdpSocket> {
    pub socket: S,
    connected: bool,
} impl<S: ToUdpSocket> Client<S> {
    pub fn new(socket: S) -> Self {
        Self {
            socket,
            connected: false,
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

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn connect_to_master<A: ToSocketAddrs>(&mut self, addr: A) -> Result<(), A2SClientError> {
        if !self.connected {
            match self.socket.conn(addr) {
                Ok(_) => self.connected = true,
                Err(error) => return Err(A2SClientError::IoError(error)),
            };
        } else {
            return Err(A2SClientError::ToMasterConnectionRepeated)
        }

        Ok(())
    }

    fn push_to_payload<'a, T>(vec: &mut Vec<u8>, list: T)
        where T: IntoIterator<Item=&'a u8> {
        for b in list.into_iter(){
            vec.push(*b)
        }
    }

    fn build_packet<'a, T>(host: T, region: &Regions, filter: &Filter) -> Vec<u8>
        where T: IntoIterator<Item=&'a u8> {
        let mut payload: Vec<u8> = Vec::new();
        payload.push(MESSAGE_TYPE);
        payload.push(*region as u8);
        Self::push_to_payload(&mut payload, host);
        payload.push(NULL_BYTE);
        Self::push_to_payload(&mut payload, filter.to_bytes().iter());
        payload.push(NULL_BYTE);
        payload
    }

    fn parse2(host: &[u8]) -> (String, u16) {
        let pre = 0;

        let mut ip = String::new();
        for x in &host[pre..pre+4] {
            ip.push_str(&*(
                u8::from_str_radix(
                    &*format!("{:X}", *x),
                    16
                ).unwrap()
            ).to_string()

            );
            ip.push('.');
        }
        ip = ip.strip_suffix('.').unwrap().parse().unwrap();

        let port = u16::from_str_radix(
            &*format!("{:X}{:X}", &host[pre+4], &host[pre+5]),
            16
        ).unwrap();

        return (ip, port)
    }

    // Unfinished function [debug]
    // ToDo change the output
    // ToDo remove comments
    // ToDo add user defined timeout
    pub fn get_servers<'a>(&self, region: Regions, filter: Filter, f: fn(String, u16)) -> bool {
        //                             const   region  0     .     0     .     0     .     0    :      0    \0   \0
        //         let payload = [MESSAGE_TYPE, 0xFF, 0x30, 0x2E, 0x30, 0x2E, 0x30, 0x2E, 0x30, 0x3A, 0x30, 00, 00_u8];

        *HOST.lock().unwrap() = DEFAULT_HOST.to_vec();
        loop {

            // println!("Sending packet with host: {:X?}", HOST.lock().unwrap());
            let packet = Self::build_packet(HOST.lock().unwrap().iter(), &region, &filter);
            // println!("Sending...");
            self.socket.send_packet(&packet).unwrap();
            // println!("SENT!");

            let mut buf = [0u8; 4096];
            // println!("Waiting for the response...");
            let (size, response) = match self.socket.receive_packet(&mut buf) {
                Ok(received) => {
                    println!("[MASTER SERVER] received {} bytes", received);
                    (received-6, &buf[6..received])
                },
                Err(_) => {
                    // println!("[MASTER SERVER] recv function failed: {:?}", e);
                    break;
                },
            };

            for x in 0..(response.len()/6)-6 {
                let pre = x*6;
                let (ip, port) = Self::parse2(&response[pre..pre+6]);

                f(ip, port);
            }

            let (ip, port) = Self::parse2(&response[size-6..]);
            let bytes = format!("{}:{}", ip, port).into_bytes();
            if bytes.as_slice() != DEFAULT_HOST {
                *HOST.lock().unwrap() = bytes
            } else {
                println!("[MASTER SERVER] End of the hosts list!");
                break;
            }

            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
        true
    }
}
