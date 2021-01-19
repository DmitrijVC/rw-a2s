use std::convert::From;
use std::ops::AddAssign;

use crate::types::Bool;


#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Regions {
    USEastCoast = 0x00,
    USWestCoast = 0x01,
    SouthAmerica = 0x02,
    Europe = 0x03,
    Asia = 0x04,
    Australia = 0x05,
    MiddleEast = 0x06,
    Africa = 0x07,
    All = 0xFF,
}

macro_rules! filter_enum {
    ($($enum_value:ident($value_type:ty) = $enum_value_name:expr),+) => {
        pub enum FilterCode {
            $( $enum_value($value_type) ),+
        }
        impl From<FilterCode> for String {
            fn from(filter: FilterCode) -> Self {
                match filter {
                    $( FilterCode::$enum_value(value) => format!("\\{}\\{}", $enum_value_name, value) ),+
                }
            }
        }
    };
}

filter_enum!(
    Dedicated(Bool) = "dedicated",
    Secure(Bool) = "secure",
    Linux(Bool) = "linux",
    Password(Bool) = "password",
    Empty(Bool) = "empty",
    Full(Bool) = "full",
    NoPlayers(Bool) = "noplayers",
    White(Bool) = "white",
    CollapseAddrHash(Bool) = "collapse_addr_hash",
    GameDir(String) = "gamedir",
    Map(String) = "map",
    GameAddr(String) = "gameaddr",
    AppId(u32) = "appid",
    NoAppId(u32) = "napp"
);


#[derive(Debug)]
pub struct Filter{
    pub data: String,
} impl Filter {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            data: String::new(),
        }
    }

    fn is_null(&self) -> bool {
        self.data.is_empty()
    }

    #[allow(dead_code)]
    pub fn add(mut self, code: FilterCode) -> Self {
        self._add(code);
        self
    }

    fn _add(&mut self, code: FilterCode) {
        let full_filter: String = code.into();

        if self.is_null() {
            self.data = full_filter;
        } else {
            self.data.push_str(&full_filter);
        }
    }

    pub fn to_bytes(&self) -> Vec<u8>{
        self.data.clone().into_bytes()
    }

}

impl Default for Filter {
    fn default() -> Self {
        Self {
            data: 0x00.to_string()
        }
    }
}

impl AddAssign<FilterCode> for Filter {
    fn add_assign(&mut self, rhs: FilterCode) {
        self._add(rhs);
    }
}

impl From<&str> for Filter {
    fn from(raw: &str) -> Self {
        Self {
            data: String::from(raw)
        }
    }
}
