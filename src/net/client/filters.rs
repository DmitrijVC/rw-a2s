use crate::types::Bool;
use crate::errors::FilterError;
use std::any::TypeId;
use std::fmt::Display;


pub enum Regions {
    USEastCoast,
    USWestCoast,
    SouthAmerica,
    Europe,
    Asia,
    Australia,
    MiddleEast,
    Africa,
    All,
} impl Regions {
    pub fn byte(&self) -> u8 {
        match *self {
            Regions::USEastCoast => 0x00,
            Regions::USWestCoast => 0x01,
            Regions::SouthAmerica => 0x02,
            Regions::Europe => 0x03,
            Regions::Asia => 0x04,
            Regions::Australia => 0x05,
            Regions::MiddleEast => 0x06,
            Regions::Africa => 0x07,
            Regions::All => 0xFF,
        }
    }
}

#[allow(dead_code)]
pub enum FilterCode{
    Dedicated,
    Secure,
    Linux,
    Password,
    Empty,
    Full,
    NoPlayers,
    White,
    CollapseAddrHash,
    GameDir,
    Map,
    GameAddr,
    AppId,
    NoAppId,
} impl FilterCode {
    pub fn get_type(&self) -> TypeId {
        match *self {
            FilterCode::Dedicated => TypeId::of::<&Bool>(),
            FilterCode::Secure => TypeId::of::<&Bool>(),
            FilterCode::Linux => TypeId::of::<&Bool>(),
            FilterCode::Password => TypeId::of::<&Bool>(),
            FilterCode::Empty => TypeId::of::<&Bool>(),
            FilterCode::Full => TypeId::of::<&Bool>(),
            FilterCode::NoPlayers => TypeId::of::<&Bool>(),
            FilterCode::White => TypeId::of::<&Bool>(),
            FilterCode::CollapseAddrHash => TypeId::of::<&Bool>(),
            FilterCode::GameDir => TypeId::of::<&&str>(),
            FilterCode::Map => TypeId::of::<&&str>(),
            FilterCode::GameAddr => TypeId::of::<&&str>(),
            FilterCode::AppId => TypeId::of::<&u16>(),
            FilterCode::NoAppId => TypeId::of::<&u16>(),
        }
    }

    pub fn get_name(&self) -> &str {
        match *self {
            FilterCode::Dedicated => "dedicated",
            FilterCode::Secure => "secure",
            FilterCode::Linux => "linux",
            FilterCode::Password => "password",
            FilterCode::Empty => "empty",
            FilterCode::Full => "full",
            FilterCode::NoPlayers => "noplayers",
            FilterCode::White => "white",
            FilterCode::CollapseAddrHash => "collapse_addr_hash",
            FilterCode::GameDir => "gamedir",
            FilterCode::Map => "map",
            FilterCode::GameAddr => "gameaddr",
            FilterCode::AppId => "appid",
            FilterCode::NoAppId => "napp",
        }
    }
}

#[derive(Debug)]
pub struct Filter{
    pub data: String,
} impl Filter {
    #[allow(dead_code)]
    pub fn new(raw_filter: Option<&str>) -> Self {
        Self {
            data: match raw_filter {
                None => String::new(),
                Some(result) => String::from(result),
            },
        }
    }

    fn is_null(&self) -> bool {
        self.data.is_empty()
    }

    #[allow(dead_code)]
    pub fn add_unchecked<T>(&mut self, code: FilterCode, value: &'static T) -> &mut Self
        where T: Display {
        if code.get_type() == TypeId::of::<&'static T>() {

            let full_filter = format!(r"\{}\{}", code.get_name(), value);

            if self.is_null() {
                self.data = full_filter;
            } else {
                self.data.push_str(&*full_filter)
            }

        }

        self
    }

    pub fn add<T>(&mut self, code: FilterCode, value: &'static T) -> Result<&mut Self, FilterError>
        where T: Display {
        if code.get_type() == TypeId::of::<&'static T>() {

            let full_filter = format!(r"\{}\{}", code.get_name(), value);

            if self.is_null() {
                self.data = full_filter;
            } else {
                self.data.push_str(&*full_filter)
            }

        } else {
            return Err(FilterError::InvalidValueType)
        }

        Ok(self)
    }

    pub fn to_bytes(&self) -> Vec<u8>{
        self.data.clone().into_bytes()
    }

} impl Default for Filter {
    fn default() -> Self {
        Self {
            data: 0x00.to_string()
        }
    }
}
