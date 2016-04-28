// Copyright 2016, NICTA
//
// This software may be distributed and modified according to the terms of
// the BSD 2-Clause license. Note that NO WARRANTY is provided.
// See "LICENSE_BSD2.txt" for details.
//
// @TAG(NICTA_BSD)
//

extern crate compiler_rt;

use std::env;
use std::path::Path;

/// This is uses environment variables that are provided by the seL4 build system
fn main() {
    let rust_dir = &env::var("RUST_SRC").expect("RUST_SRC env var");
    let out_dir = format!("{}",
                          Path::new(&env::var("OUT_DIR").expect("OUT_DIR env var"))
                              .parent()
                              .and_then(|a| a.parent().and_then(|b| b.parent()))
                              .expect("No parent for outdir")
                              .display());
    let llvm_triple = &env::var("RUST_LLVM_TRIPLE").expect("OUT_DIR env var");
    env::set_var("CC", "gcc");
    env::set_var("AR", "ar");
    env::set_var("CXX", "g++");
    env::set_var("CPP", "cpp");
    env::set_var("HOSTCC", "gcc");

    env::set_var("CFLAGS", "");
    env::set_var("CXXFLAGS", "");
    env::set_var("LDFLAGS", "");
    compiler_rt::build_compiler_rt(rust_dir, &out_dir, llvm_triple)
}
