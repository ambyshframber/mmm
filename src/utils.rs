use std::iter::IntoIterator;
use thiserror::Error;
use midir::*;
use std::thread::sleep;
use std::time::Duration;
use std::num::ParseIntError;

pub const CLIENT_NAME: &str = "MidiMappingManager";

pub fn shortened_keyword_match<'a, T, S>(kwd: &str, options: T) -> Option<usize> 
where T: IntoIterator<Item = S>, S: AsRef<str> {
    let mut ret = 0;
    let mut found_match = false;

    for (idx, word) in options.into_iter().enumerate() {
        let word = word.as_ref();
        if word.starts_with(kwd) {
            if word == kwd {
                return Some(idx)
            }
            else if found_match {
                return None
            }
            else {
                found_match = true;
                ret = idx
            }
        }
    }

    if found_match {
        Some(ret)
    }
    else {
        None
    }
}

pub fn sleep_ms(ms: u64) {
    sleep(Duration::from_millis(ms))
}

pub fn push_if_not_present<T>(val: T, vec: &mut Vec<T>)
where T: PartialEq {
    if !vec.iter().any(|v| *v == val ) {
        vec.push(val)
    }
}

pub type Id = u32;
pub type Result<T> = std::result::Result<T, MMMErr>;

#[derive(Error, Debug)]
pub enum MMMErr {
    #[error("midi initialisation failure")]
    InitFailure(#[from] InitError),
    #[error("input connection failure")]
    InputFailure(#[from] ConnectError<MidiInput>),
    #[error("output connection failure")]
    OutputFailure(#[from] ConnectError<MidiOutput>),
    #[error("port information failure")]
    PortInfoFailure(#[from] PortInfoError),
    #[error("argument error")]
    ArgError,
    #[error("parse error")]
    ParseError(#[from] ParseIntError),
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MidiMessage {
    ts: u64,
    data: MidiMessageKind
}
#[derive(Clone, Debug)]
pub enum MidiMessageKind {
    Channel([u8; 3]), // always the same length
    ChannelSmall([u8; 2]),
    SystemCommon(Vec<u8>), // less common, so the heap allocations are less of a hit
    SystemRealtime(u8) // always 1 byte
}
impl MidiMessage {
    pub fn from_slice(ts: u64, bytes: &[u8]) -> Option<MidiMessage> {
        if bytes[0] < 0b1111_0000 { // channel message
            if bytes.len() == 3 {
                let array = bytes.try_into().unwrap();
                Some(MidiMessage {
                    ts,
                    data: MidiMessageKind::Channel(array)
                })
            }
            else if bytes.len() == 2 {
                let array = bytes.try_into().unwrap();
                Some(MidiMessage {
                    ts,
                    data: MidiMessageKind::ChannelSmall(array)
                })
            }
            else {
                None
            }
        }
        else if bytes[0] < 0b1111_1000 { // common message
            let v = bytes.to_vec();
            Some(MidiMessage {
                ts,
                data: MidiMessageKind::SystemCommon(v)
            })
        }
        else { // realtime
            Some(MidiMessage {
                ts,
                data: MidiMessageKind::SystemRealtime(bytes[0])
            })
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        type MMK = MidiMessageKind;
        if let MMK::SystemCommon(v) = &self.data {
            v.clone()
        }
        else {
            let mut ret = Vec::new();
            match self.data {
                MMK::Channel(bytes) => ret.extend(bytes),
                MMK::ChannelSmall(bytes) => ret.extend(bytes),
                MMK::SystemRealtime(b) => ret.push(b),
                _ => unreachable!()
            }
            ret
        }
    }

    /// returns None for non-channel message. goes by midi channel number, not binary value (the lowest channel is 1)
    pub fn channel(&self) -> Option<u8> {
        type MMK = MidiMessageKind;
        match self.data {
            MMK::Channel(b) => Some(b[0]),
            MMK::ChannelSmall(b) => Some(b[0]),
            _ => None
        }.map(|b| (b & 0xf) + 1)
    }
    /// goes by midi channel number, not binary value (the lowest channel is 1)
    pub fn with_channel(&self, channel: u8) -> MidiMessage {
        assert!((1..=16).contains(&channel));
        let mut ret = self.clone();
        ret.data = ret.data.with_channel(channel);
        ret
    }
}
impl MidiMessageKind {
    pub fn with_channel(self, channel: u8) -> MidiMessageKind {
        match self {
            Self::Channel(mut b) => {
                let c_actual = channel - 1;
                b[0] &= 0xf0;
                b[0] |= c_actual;
                Self::Channel(b)
            }
            Self::ChannelSmall(mut b) => {
                let c_actual = channel - 1;
                b[0] &= 0xf0;
                b[0] |= c_actual;
                Self::ChannelSmall(b)
            }
            _ => self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEYWORDS: &[&str] = &["select", "delete", "series", "remove"];
    #[test]
    fn test_kwd_match() {
        assert_eq!(shortened_keyword_match("sel", KEYWORDS), Some(0));
        assert_eq!(shortened_keyword_match("se", KEYWORDS), None);
        assert_eq!(shortened_keyword_match("delete", KEYWORDS), Some(1));
        assert_eq!(shortened_keyword_match("d", KEYWORDS), Some(1));
        assert_eq!(shortened_keyword_match("r", KEYWORDS), Some(3));
        assert_eq!(shortened_keyword_match("a", KEYWORDS), None);
    }

    #[test]
    fn midi_channels() {
        let m = MidiMessage::from_slice(0, &[0b1001_0000, 69, 69]).unwrap();
        assert_eq!(m.channel(), Some(1));
        assert_eq!(m.with_channel(8).channel(), Some(8));
        
    }
}
