// Copyright 2014-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use target::Target;

pub fn target() -> Target {
    let mut base = super::openbsd_base::opts();
    base.cpu = "x86-64".to_string();
    base.pre_link_args.push("-m64".to_string());

    Target {
        llvm_target: "x86_64-unknown-openbsd".to_string(),
        target_endian: "little".to_string(),
        target_pointer_width: "64".to_string(),
        data_layout: "e-m:e-i64:64-f80:128-n8:16:32:64-S128".to_string(),
        arch: "x86_64".to_string(),
        target_os: "openbsd".to_string(),
        target_env: "".to_string(),
        target_vendor: "unknown".to_string(),
        options: base,
    }
}
