pub mod commands {
	pub const COMMANDS: &[&str] = &["exit", "list", "ls", "rename", "connect", "cfg", "init", "new", "remove", "inputs"];
	pub const IDX_EXIT: usize = 0;
	pub const IDX_LIST: usize = 1;
	pub const IDX_LS: usize = 2;
	pub const IDX_RENAME: usize = 3;
	pub const IDX_CONNECT: usize = 4;
	pub const IDX_CFG: usize = 5;
	pub const IDX_INIT: usize = 6;
	pub const IDX_NEW: usize = 7;
	pub const IDX_REMOVE: usize = 8;
	pub const IDX_INPUTS: usize = 9;
}

pub mod processors {
	pub const PROCESSORS: &[&str] = &["input", "output", "channelfilter"];
	pub const IDX_INPUT: usize = 0;
	pub const IDX_OUTPUT: usize = 1;
	pub const IDX_CHANNELFILTER: usize = 2;
}

pub mod processor_ctors {
	pub const PROCESSOR_CTORS: &[fn(String, &[String]) -> crate::utils::Result<Box<dyn crate::MidiIO>>] = &[crate::processors::connection::MidiIn::new_args, crate::processors::connection::MidiOut::new_args, crate::processors::channelfilter::ChannelFilter::new_args, ];
}

