// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// xfail-win32

extern mod std;

use std::list::{List, Cons, Nil};
use std::time::precise_time_s;

enum UniqueList {
    ULNil, ULCons(~UniqueList)
}

fn main() {
    let (repeat, depth) = if os::getenv(~"RUST_BENCH").is_some() {
        (50, 1000)
    } else {
        (10, 10)
    };

    run(repeat, depth);
}

fn run(repeat: int, depth: int) {
    for iter::repeat(repeat as uint) {
        debug!("starting %.4f", precise_time_s());
        do task::try {
            recurse_or_fail(depth, None)
        };
        debug!("stopping %.4f", precise_time_s());
    }
}

type nillist = List<()>;

// Filled with things that have to be unwound

struct State {
    box: @nillist,
    unique: ~nillist,
    fn_box: fn@() -> @nillist,
    tuple: (@nillist, ~nillist),
    vec: ~[@nillist],
    res: r
}

struct r {
  _l: @nillist,
}

impl r : Drop {
    fn finalize(&self) {}
}

fn r(l: @nillist) -> r {
    r {
        _l: l
    }
}

fn recurse_or_fail(depth: int, st: Option<State>) {
    if depth == 0 {
        debug!("unwinding %.4f", precise_time_s());
        die!();
    } else {
        let depth = depth - 1;

        let st = match st {
          None => {
            State {
                box: @Nil,
                unique: ~Nil,
                fn_box: fn@() -> @nillist { @Nil::<()> },
                tuple: (@Nil, ~Nil),
                vec: ~[@Nil],
                res: r(@Nil)
            }
          }
          Some(st) => {
            let fn_box = st.fn_box;

            State {
                box: @Cons((), st.box),
                unique: ~Cons((), @*st.unique),
                fn_box: fn@() -> @nillist { @Cons((), fn_box()) },
                tuple: (@Cons((), st.tuple.first()),
                        ~Cons((), @*st.tuple.second())),
                vec: st.vec + ~[@Cons((), st.vec.last())],
                res: r(@Cons((), st.res._l))
            }
          }
        };

        recurse_or_fail(depth, Some(move st));
    }
}
