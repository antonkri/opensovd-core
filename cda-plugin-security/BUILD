# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2026 The Contributors to Eclipse OpenSOVD (see CONTRIBUTORS)

load("@cda_crates//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_rust//rust:defs.bzl", "rust_library")

rust_library(
    name = "cda-plugin-security",
    srcs = glob(["src/**/*.rs"]),
    aliases = aliases(),
    crate_name = "cda_plugin_security",
    edition = "2024",
    proc_macro_deps = all_crate_deps(
        normal_dev = False,
        proc_macro = True,
    ),
    visibility = ["//visibility:public"],
    deps = all_crate_deps(normal = True) + [
        "//cda-database:cda-database",
        "//cda-interfaces:cda-interfaces",
        "//cda-sovd-interfaces:cda-sovd-interfaces",
    ],
)

exports_files(["Cargo.toml"])
