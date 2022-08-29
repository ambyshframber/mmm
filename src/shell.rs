use std::sync::{Mutex, Arc};
use std::thread::spawn;
use rustyline::Editor;
use crate::utils::*;

pub struct Messenger {
    pub shell_wait: bool,
    command: Option<String>
}
impl Messenger {
    pub fn new() -> Messenger {
        Messenger {
            shell_wait: false,
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
    msgr: Arc<Mutex<Messenger>>
}
impl Shell {
    pub fn new() -> Arc<Mutex<Messenger>> {
        let msgr = Arc::new(Mutex::new(Messenger::new()));
        let msgr_ret = Arc::clone(&msgr);

        let mut shell = Shell {
            msgr
        };

        let _thread = spawn(move || shell.run());

        msgr_ret
    }
    pub fn run(&mut self) {
        loop {
            sleep_ms(10);
            let msgr = self.msgr.lock().unwrap();
            if msgr.shell_wait {
                continue
            }
            else {
                std::mem::drop(msgr);
                let mut editor: Editor<()> = Editor::new().unwrap();
                if let Ok(line) = editor.readline("> ") {
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

