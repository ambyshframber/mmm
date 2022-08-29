#!/bin/env python3

kwds_main = {
    "commands": ["exit", "list", "rename"]
}

for mod, kwds in kwds_main.items():
    print(f"pub mod {mod} {{")
    kwds_str = str(kwds).replace("'", '"')
    print(f"\tpub const KWDS: &[&str] = &{kwds_str};")

    for i, k in enumerate(kwds):
        print(f"\tpub const IDX_{k.upper()}: usize = {i};")

    print("}\n")
