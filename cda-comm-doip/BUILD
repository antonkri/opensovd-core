# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2026 The Contributors to Eclipse OpenSOVD (see CONTRIBUTORS)

load("@cda_crates//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_rust//cargo:defs.bzl", "cargo_build_script")
load("@rules_rust//rust:defs.bzl", "rust_library")

# build.rs only prints feature-conflict warnings; no code generation.
cargo_build_script(
    name = "cda_comm_doip_build_script",
    srcs = ["build.rs"],
    crate_features = [
        # Default: openssl (not mbedtls).  Build script just emits a warning if
        # both are active, which they are not in the default configuration.
        "openssl",
    ],
    crate_name = "build_script_build",
    edition = "2024",
    deps = all_crate_deps(build = True),
)

rust_library(
    name = "cda-comm-doip",
    srcs = glob(["src/**/*.rs"]),
    aliases = aliases(),
    # Use openssl TLS (default).  To use mbedtls instead, change to "mbedtls"
    # and uncomment the mbedtls crates in MODULE.bazel.
    crate_features = ["openssl"],
    crate_name = "cda_comm_doip",
    edition = "2024",
    proc_macro_deps = all_crate_deps(
        normal_dev = False,
        proc_macro = True,
    ),
    visibility = ["//visibility:public"],
    deps = all_crate_deps(
        normal = True,
    ) + [
        ":cda_comm_doip_build_script",
        "//cda-interfaces:cda-interfaces",
    ],
)

exports_files(["Cargo.toml"])
