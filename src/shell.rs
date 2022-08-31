use std::sync::{Mutex, Arc};
use std::thread::spawn;
use rustyline::Editor;
use crate::utils::*;

pub struct Messenger {
    pub shell_wait: bool,
    pub exiting: bool,
    command: Option<String>
}
impl Messenger {
    pub fn new() -> Messenger {
        Messenger {
            shell_wait: false,
            exiting: false,
            command: None
        }
    }
    pub fn set_message(&mut self, msg: String) {
        self.shell_wait = true;
        self.command = Some(msg)
    }
    pub fn read_message(&mut self) -> Option<String> {
        self.command.take()
    }
}
pub struct Shell {
    msgr: Arc<Mutex<Messenger>>,
    rl: Editor<()>
}
impl Shell {
    pub fn new() -> (Arc<Mutex<Messenger>>, std::thread::JoinHandle<()>) {
        let msgr = Arc::new(Mutex::new(Messenger::new()));
        let msgr_ret = Arc::clone(&msgr);

        let mut shell = Shell {
            msgr,
            rl: Editor::new().unwrap()
        };

        let thread = spawn(move || shell.run());

        (msgr_ret, thread)
    }
    pub fn run(&mut self) {
        loop {
            sleep_ms(10);
            let msgr = self.msgr.lock().unwrap();
            if msgr.shell_wait {
                continue
            }
            if msgr.exiting {
                break
            }
            else {
                std::mem::drop(msgr);
                if let Ok(line) = self.rl.readline("> ") {
                    if line.is_empty() {
                        continue
                    }
                    let mut msgr = self.msgr.lock().unwrap();
                    msgr.set_message(line)
                }
            }
        }
    }
}

