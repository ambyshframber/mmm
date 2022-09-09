use crate::utils::*;
use crate::consts::channelfilter_cmds::*;
use crate::MidiIO;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct DummyPlayer {
    channel: u8,
    name: String,
    outputs: Vec<Id>,
    last_msg: u64
}
impl DummyPlayer {
    fn new(channel: u8, name: String) -> DummyPlayer {
        DummyPlayer {
            channel, name,
            outputs: Vec::new(),
            last_msg: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
        }
    }
    pub fn new_args(name: String, args: &[String]) -> Result<Box<dyn MidiIO>> {
        if args.len() != 1 {
            Err(MMMErr::ArgError)
        }
        else {
            let c: u8 = args[0].parse()?;
            if (1..=16).contains(&c) {
                Ok(Box::new(Self::new(c, name)) as Box<dyn MidiIO>)
            }
            else {
                Err(MMMErr::ArgError)
            }
        }
    }

    pub fn change_channel(&mut self, args: &[String]) {
        if args.is_empty() {
            println!("channel number required")
        }
        else {
            if let Ok(channel) = args[0].parse() {
                self.channel = channel
            }
            else {
                println!("channel number failed to parse")
            }
        }
    }
}
impl MidiIO for DummyPlayer {
    fn can_read(&self) -> bool { true }
    fn can_write(&self) -> bool { false }

    fn get_name(&self) -> String { self.name.clone() }
    fn get_display_name(&self) -> String { format!("{} (dummyplayer)", self.name) }
    fn set_name(&mut self, name: &str) { self.name = name.into() }

    fn list_outputs(&self) -> &[Id] { &self.outputs }
    fn add_output(&mut self, id: Id) {
        push_if_not_present(id, &mut self.outputs)
    }
    fn rem_output(&mut self, id: Id) {
        self.outputs.iter().position(|i| *i == id).map(|idx| self.outputs.remove(idx));
    }

    fn cfg(&mut self, command: &[String]) {
        if !command.is_empty() {
            println!("dummyplayer on channel {}", self.channel)
        }
        else {
            match shortened_keyword_match(&command[0], CHANNELFILTER_CMDS) {
                Some(IDX_CHANNEL) => self.change_channel(&command[1..]),
                _ => {
                    println!("command not found! valid commands are:");
                    for cmd in CHANNELFILTER_CMDS {
                        println!("\t{}", cmd)
                    }
                }
            }
        }
    }    

    fn write(&mut self, _messages: &[MidiMessage]) { unreachable!() }
    fn read(&mut self) -> Vec<MidiMessage> {
        let mut ret = Vec::new();

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        if now > self.last_msg {
            ret.push(MidiMessage::Channel([0x90 | self.channel, 0x40, 0x40]));
            self.last_msg = now
        }

        ret
    }
    
    fn delete(self) { }
}
