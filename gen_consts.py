#!/bin/env python3

kwds_main = [
    ("commands", [
        "exit",
        "list", "ls",
        "rename",
        "connect", "disconnect",
        "cfg",
        "init", "new",
        "remove",
        "inputs",
        "outputs"
    ]),

    ("processors", [
        "input", "output",
        "channelfilter",
    ]),

    (["processor_ctors", "&[fn(String, &[String]) -> crate::utils::Result<Box<dyn crate::MidiIO>>]"], [
        "crate::processors::connection::MidiIn::new_args",
        "crate::processors::connection::MidiOut::new_args",
        "crate::processors::channelfilter::ChannelFilter::new_args",
    ]),
]

def list_to_string(l, print_quotes):
    if print_quotes:
        return str(l).replace("'", '"')
    else:
        buf = "["
        for i in l:
            buf += f"{i}, "
        buf += "]"
        return buf

def ctls(l):
    if type(l) == type(""):
        return (l, "&[&str]")
    else:
        return (l[0], l[1])


for ctl, kwds in kwds_main:
    (mod, ty) = ctls(ctl)
    print(f"pub mod {mod} {{")
    kwds_str = list_to_string(kwds, ty == "&[&str]")
    print(f"\tpub const {mod.upper()}: {ty} = &{kwds_str};")

    if ty == "&[&str]":
        for i, k in enumerate(kwds):
            print(f"\tpub const IDX_{k.upper()}: usize = {i};")
        
    print("}\n")
