/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate cc;

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    let windows_hidpi = if cfg!(feature = "windows-hidpi") {
        "USE_WINDOWS_HIDPI"
    }else {
        "NO_WINDOWS_HIDPI"
    };
    
    cc::Build::new()
        .file("libtinyfiledialogs/tinyfiledialogs.c")
        .flag("-v")
        .define(windows_hidpi,None)
        .compile("libtinyfiledialogs.a");

    if target.contains("windows") {
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=comdlg32");
        println!("cargo:rustc-link-lib=ole32");
        println!("cargo:rustc-link-lib=shell32");
        if !target.ends_with("pc-windows-gnu") {
            println!("cargo:rustc-link-lib=shcore");
        }
    }
}
