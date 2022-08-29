use std::collections::HashMap;
use utils::*;
use connection::*;
use shell::*;
use std::sync::{Arc, Mutex};
use shell_words::split;

mod utils;
mod connection;
mod shell;
mod consts;

fn main() {
    let mut mm = MidiManager::new();
    mm.test()
}

struct MidiManager {
    map: HashMap<Id, Box<dyn MidiIO>>,
    id_ctr: Id,
    returned_ids: Vec<Id>,
    msgr: Arc<Mutex<Messenger>>
}
impl MidiManager {
    pub fn new() -> MidiManager {
        let msgr = Shell::new();
        MidiManager {
            map: HashMap::new(),
            id_ctr: 0,
            returned_ids: Vec::new(),
            msgr
        }
    }
    fn next_id(&mut self) -> Id {
        if let Some(i) = self.returned_ids.pop() {
            i
        }
        else {
            if self.id_ctr == Id::MAX {
                panic!("out of ids!")
            }
            let i = self.id_ctr;
            self.id_ctr += 1;
            i
        }
    }
    fn new_in(&mut self, idx: usize) -> Result<Id> {
        let id = self.next_id();
        let in_ = Box::new(MidiIn::new(idx, id)?);
        self.map.insert(id, in_);
        Ok(id)
    }
    fn new_out(&mut self) -> Result<Id> {
        let id = self.next_id();
        let out = Box::new(MidiOut::new(id)?);
        self.map.insert(id, out);
        Ok(id)
    }
    fn update_map(&mut self) {
        //sleep_ms(1);
        let all_ids: Vec<Id> = self.map.keys().map(|id| *id).collect();
        for id in all_ids {
            let vp = self.map.get_mut(&id).unwrap();
            if vp.can_read() {
                let msgs = vp.read();
                let outputs = vp.list_outputs().to_vec();
                for o in outputs {
                    if let Some(dest) = self.map.get_mut(&o) {
                        dest.write(&msgs)
                    }
                }
            }
        }
    }
    fn do_command(&mut self, command: &str) {
        if let Ok(parts) = split(command) {
            
        }
        let mut msgr = self.msgr.lock().unwrap();
        msgr.shell_wait = false;
    }

    pub fn test(&mut self) {
        let in_id = self.new_in(1).unwrap();
        let out_id = self.new_out().unwrap();

        let in_ = self.map.get_mut(&in_id).unwrap();
        in_.add_output(out_id);
        loop {
            self.update_map();
            let mut msgr = self.msgr.lock().unwrap();
            let msg = msgr.read_message();
            drop(msgr);
            if let Some(cmd) = msg {
                self.do_command(&cmd)
            }
        }
    }
}

trait MidiIO {
    fn can_read(&self) -> bool;
    fn can_write(&self) -> bool;

    fn add_output(&mut self, id: Id);
    fn rem_output(&mut self, id: Id);
    fn list_outputs(&self) -> &[Id];

    fn get_name(&self) -> String;
    fn set_name(&mut self, name: &str);

    fn control(&mut self, command: &str) -> String;
    
    fn write(&mut self, messages: &[MidiMessage]);
    fn read(&mut self) -> Vec<MidiMessage>;
}
