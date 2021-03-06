// xfail-fast

// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Minimized version of issue-2804.rs. Both check that callee IDs don't
// clobber the previous node ID in a macro expr
extern mod std;
use std::oldmap::HashMap;

fn add_interfaces(managed_ip: ~str, device: std::oldmap::HashMap<~str, int>)  {
     error!("%s, %?", managed_ip, device[~"interfaces"]);
}

pub fn main() {}
