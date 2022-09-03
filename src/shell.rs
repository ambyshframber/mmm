use std::sync::{Mutex, Arc};
use std::thread;
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::process::Command;
use rustyline::Editor;
use shell_words::split;
use crate::utils::*;
use crate::consts::metacommands::*;

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
    rl: Editor<()>,
    int_buf: VecDeque<String>
}
impl Shell {
    pub fn new() -> (Arc<Mutex<Messenger>>, std::thread::JoinHandle<()>) {
        let msgr = Arc::new(Mutex::new(Messenger::new()));
        let msgr_ret = Arc::clone(&msgr);

        let mut shell = Shell {
            msgr,
            rl: Editor::new().unwrap(),
            int_buf: VecDeque::new()
        };

        let thread = thread::Builder::new().name(String::from("shell")).spawn(move || shell.run()).unwrap();

        (msgr_ret, thread)
    }
    fn run(&mut self) {
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
                if self.int_buf.is_empty() {
                    if let Ok(line) = self.rl.readline("> ") {
                        self.do_line(line)
                    }
                }
                else {
                    self.send_msg()
                }
            }
        }
    }
    fn do_line(&mut self, line: String) {
        if line.is_empty() { return }
        if line.starts_with('.') {
            if line.len() > 1 {
                self.do_command(&line[1..])
            }
        }
        else {
            self.int_buf.push_back(line)
        }
    }
    fn do_command(&mut self, command: &str) {
        if let Ok(s) = split(command) {
            let command = &s[0];
            let args = &s[1..];
            if let Some(idx) = shortened_keyword_match(command, METACOMMANDS) {
                match idx {
                    IDX_LOAD => self.load(args),
                    IDX_RUN => self.run_mc(args),
                    _ => unreachable!()
                }
            }
            else {
                println!("command not found! valid metacommands are:");
                for c in METACOMMANDS {
                    println!("\t{}", c)
                }
            }
        }
    }
    fn send_msg(&mut self) {
        if let Some(line) = self.int_buf.pop_front() {
            let mut msgr = self.msgr.lock().unwrap();
            msgr.set_message(line);
        }
    }

    fn run_mc(&mut self, args: &[String]) {
        if args.is_empty() {
            println!("run metacommand requires at least 1 argument")
        }
        else {
            if let Ok(output) = Command::new(&args[0]).args(&args[1..]).output() {
                println!("{}", String::from_utf8_lossy(&output.stderr));
                if let Ok(s) = std::str::from_utf8(&output.stdout) {    
                    for line in s.split('\n') {
                        self.do_line(line.into());
                    }
                }
                else {
                    println!("command returned invalid utf8")
                }
            }
            else {
                println!("failed to run command!")
            }
        }
    }
    fn load(&mut self, args: &[String]) {
        if args.len() != 1 {
            println!("load metacommand requires 1 argument")
        }
        else {
            if let Ok(s) = read_to_string(&args[0]) {
                for line in s.split('\n') {
                    self.do_line(line.into());
                }
            }
            else {
                println!("failed to read file")
            }
        }
    }
}

