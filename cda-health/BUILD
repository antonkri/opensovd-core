# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2026 The Contributors to Eclipse OpenSOVD (see CONTRIBUTORS)

load("@cda_crates//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_rust//rust:defs.bzl", "rust_library")

rust_library(
    name = "cda-health",
    srcs = glob(["src/**/*.rs"]),
    aliases = aliases(),
    crate_name = "cda_health",
    edition = "2024",
    proc_macro_deps = all_crate_deps(
        normal_dev = False,
        proc_macro = True,
    ),
    visibility = ["//visibility:public"],
    deps = all_crate_deps(normal = True) + [
        "//cda-interfaces:cda-interfaces",
        "//cda-sovd:cda-sovd",
    ],
)

exports_files(["Cargo.toml"])
