# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2026 The Contributors to Eclipse OpenSOVD (see CONTRIBUTORS)

load("@cda_crates//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_rust//rust:defs.bzl", "rust_library")

# Shared type definitions and traits that every other CDA crate depends on.
rust_library(
    name = "cda-interfaces",
    srcs = glob(["src/**/*.rs"]),
    aliases = aliases(),
    crate_name = "cda_interfaces",
    edition = "2024",
    proc_macro_deps = all_crate_deps(
        normal_dev = False,
        proc_macro = True,
    ),
    visibility = ["//visibility:public"],
    deps = all_crate_deps(normal = True),
)

exports_files(["Cargo.toml"])
