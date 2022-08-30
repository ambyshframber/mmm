use crate::utils::*;
use crate::MidiIO;

pub struct ChannelFilter {
    channel: u8,
    name: String,
    buf: Vec<MidiMessage>,
    outputs: Vec<Id>,
}
impl ChannelFilter {
    fn new(channel: u8, name: String) -> ChannelFilter {
        ChannelFilter {
            channel, name,
            buf: Vec::new(),
            outputs: Vec::new()
        }
    }
    pub fn new_args(name: String, args: &[String]) -> Result<Box<dyn MidiIO>> {
        if args.len() != 1 {
            Err(MMMErr::ArgError)
        }
        else {
            let c = args[0].parse()?;
            Ok(Box::new(Self::new(c, name)) as Box<dyn MidiIO>)
        }
    }
}
impl MidiIO for ChannelFilter {
    fn can_read(&self) -> bool { true }
    fn can_write(&self) -> bool { true }

    fn get_name(&self) -> String { self.name.clone() }
    fn get_display_name(&self) -> String { format!("{} (channelfilter)", self.name) }
    fn set_name(&mut self, name: &str) { self.name = name.into() }

    fn list_outputs(&self) -> &[Id] { &self.outputs }
    fn add_output(&mut self, id: Id) {
        push_if_not_present(id, &mut self.outputs)
    }
    fn rem_output(&mut self, id: Id) {
        self.outputs.iter().position(|i| *i == id).map(|idx| self.outputs.remove(idx));
    }

    fn control(&mut self, _command: &str) -> String { unimplemented!() }

    fn write(&mut self, messages: &[MidiMessage]) {
        self.buf.extend(messages.iter().skip_while(|m| {
            m.channel().map(|c| c != self.channel).unwrap_or(false)
        }).map(|m| m.clone()))
    }
    fn read(&mut self) -> Vec<MidiMessage> {
        let replacement = Vec::new();
        std::mem::replace(&mut self.buf, replacement)
    }
    
    fn delete(self) { }
}
