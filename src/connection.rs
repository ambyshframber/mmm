use midir::{MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection, os::unix::VirtualOutput};
use std::sync::{Mutex, Arc};
use crate::{MidiIO, utils::*};
use std::mem::replace;

type MessageBuf = Arc<Mutex<Vec<MidiMessage>>>;
pub struct MidiIn {
    _connection: MidiInputConnection<MessageBuf>,
    buf: MessageBuf,
    name: String,
    port_name: String,
    outputs: Vec<Id>
}
impl MidiIn {
    pub fn new(idx: usize, id: Id) -> Result<MidiIn> {
        let name = id.to_string();
        let buf = Arc::new(Mutex::new(Vec::new()));

        let input = MidiInput::new(CLIENT_NAME)?;
        let ports = input.ports();
        let port = &ports[idx];
        let port_name = input.port_name(port)?;

        let _connection = input.connect(
            &ports[idx],
            &name,
            |ts, bytes, buf| process_msg(ts, bytes, buf),
            Arc::clone(&buf)
        )?;
        Ok(MidiIn {
            _connection, buf, name, port_name,
            outputs: Vec::new()
        })
    }
}
impl MidiIO for MidiIn {
    fn can_read(&self) -> bool { true }
    fn can_write(&self) -> bool { false }

    fn get_name(&self) -> String { format!("{} -> {}", self.port_name, self.name)}
    fn set_name(&mut self, name: &str) { self.name = name.into() }

    fn list_outputs(&self) -> &[Id] { &self.outputs }
    fn add_output(&mut self, id: Id) {
        push_if_not_present(id, &mut self.outputs)
    }
    fn rem_output(&mut self, id: Id) {
        self.outputs.iter().position(|i| *i == id).map(|idx| self.outputs.remove(idx));
    }

    fn control(&mut self, _command: &str) -> String { unimplemented!() }

    fn write(&mut self, _messages: &[MidiMessage]) { unreachable!() }
    fn read(&mut self) -> Vec<MidiMessage> {
        let replacement = Vec::new();
        let mut buf = self.buf.lock().unwrap();
        replace(&mut *buf, replacement)
    }
}
fn process_msg(ts: u64, bytes: &[u8], buf: &mut MessageBuf) {
    if let Some(msg) = MidiMessage::from_slice(ts, bytes) {
        let mut buf = buf.lock().unwrap();
        buf.push(msg)
    }
}

pub struct MidiOut {
    port: MidiOutputConnection,
    name: String,
}
impl MidiOut {
    pub fn new(id: Id) -> Result<MidiOut> {
        let name = id.to_string();

        let output = MidiOutput::new(CLIENT_NAME)?;
        let port = output.create_virtual(&format!("output id {}", id))?;

        Ok(MidiOut {
            port, name
        })
    }
}
impl MidiIO for MidiOut {
    fn can_read(&self) -> bool { false }
    fn can_write(&self) -> bool { true }

    fn get_name(&self) -> String { format!("{}", self.name)}
    fn set_name(&mut self, name: &str) { self.name = name.into() }

    fn list_outputs(&self) -> &[Id] { unreachable!() }
    fn add_output(&mut self, _id: Id) { unreachable!() }
    fn rem_output(&mut self, _id: Id) { unreachable!() }

    fn control(&mut self, _command: &str) -> String { unimplemented!() }

    fn write(&mut self, messages: &[MidiMessage]) {
        for m in messages {
            let _ = self.port.send(&m.to_bytes());
        }
    }
    fn read(&mut self) -> Vec<MidiMessage> { unreachable!() }
}
