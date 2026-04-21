# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2026 The Contributors to Eclipse OpenSOVD (see CONTRIBUTORS)

load("@cda_crates//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_rust//cargo:defs.bzl", "cargo_build_script")
load("@rules_rust//rust:defs.bzl", "rust_library")

# build.rs calls cda_build::set_nightly_flag() which emits `rustc-cfg=nightly`
# when a nightly compiler is detected.
cargo_build_script(
    name = "cda_tracing_build_script",
    srcs = ["build.rs"],
    crate_name = "build_script_build",
    edition = "2024",
    deps = [
        "//cda-build:cda-build",
    ],
)

rust_library(
    name = "cda-tracing",
    srcs = glob(["src/**/*.rs"]),
    aliases = aliases(),
    crate_name = "cda_tracing",
    edition = "2024",
    proc_macro_deps = all_crate_deps(
        normal_dev = False,
        proc_macro = True,
    ),
    visibility = ["//visibility:public"],
    deps = all_crate_deps(normal = True) + [
        ":cda_tracing_build_script",
    ],
)

exports_files(["Cargo.toml"])
