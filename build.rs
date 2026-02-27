// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright (c) 2025-2026 naskel.com

fn main() {
    let version = format!(
        "{}.{}.{}",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH"),
    );
    println!("cargo:rustc-env=HDDS_VERSION={version}");
    println!(
        "cargo:rustc-env=HDDS_BUILD_NUMBER={}",
        env!("CARGO_PKG_VERSION_PATCH")
    );
}
