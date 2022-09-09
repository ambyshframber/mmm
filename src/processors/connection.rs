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
    fn new(idx: usize, name: String) -> Result<MidiIn> {
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

    pub fn new_args(name: String, args: &[String]) -> Result<Box<dyn MidiIO>> {
        if args.len() != 1 {
            Err(MMMErr::ArgError)
        }
        else {
            let idx = args[0].parse()?;
            Self::new(idx, name).map(|m| Box::new(m) as Box<dyn MidiIO>)
        }
    }
}
impl MidiIO for MidiIn {
    fn can_read(&self) -> bool { true }
    fn can_write(&self) -> bool { false }

    fn get_display_name(&self) -> String { format!("{} -> {} (input)", self.port_name, self.name)}
    fn get_name(&self) -> String { self.name.clone() }
    fn set_name(&mut self, name: &str) { self.name = name.into() }

    fn list_outputs(&self) -> &[Id] { &self.outputs }
    fn add_output(&mut self, id: Id) {
        push_if_not_present(id, &mut self.outputs)
    }
    fn rem_output(&mut self, id: Id) {
        self.outputs.iter().position(|i| *i == id).map(|idx| self.outputs.remove(idx));
    }

    fn cfg(&mut self, _command: &[String]) { println!("n/a") }

    fn write(&mut self, _messages: &[MidiMessage]) { unreachable!() }
    fn read(&mut self) -> Vec<MidiMessage> {
        let replacement = Vec::new();
        let mut buf = self.buf.lock().unwrap();
        replace(&mut *buf, replacement)
    }
    
    fn delete(self) {
        self._connection.close();
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
    fn new(name: String) -> Result<MidiOut> {

        let output = MidiOutput::new(CLIENT_NAME)?;
        let port = output.create_virtual(&name)?;

        Ok(MidiOut {
            port, name
        })
    }
    pub fn new_args(name: String, _args: &[String]) -> Result<Box<dyn MidiIO>> {
        Self::new(name).map(|m| Box::new(m) as Box<dyn MidiIO>)
    }
}
impl MidiIO for MidiOut {
    fn can_read(&self) -> bool { false }
    fn can_write(&self) -> bool { true }

    fn get_name(&self) -> String { format!("{}", self.name)}
    fn get_display_name(&self) -> String { format!("{} (output)", self.name)}
    fn set_name(&mut self, name: &str) { self.name = name.into() }

    fn list_outputs(&self) -> &[Id] { unreachable!() }
    fn add_output(&mut self, _id: Id) { unreachable!() }
    fn rem_output(&mut self, _id: Id) { unreachable!() }

    fn cfg(&mut self, _command: &[String]) { println!("n/a") }

    fn write(&mut self, messages: &[MidiMessage]) {
        for m in messages {
            let _ = self.port.send(&m.to_bytes());
        }
    }
    fn read(&mut self) -> Vec<MidiMessage> { unreachable!() }

    fn delete(self) {}
}
